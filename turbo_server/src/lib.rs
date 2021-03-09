use futures::FutureExt as _;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use warp::{http, Filter};

mod asset_server;
mod proc_migrations;
mod schema;

pub use schema::rust_log;
pub use schema::ApplyStepsFn;
use schema::APPLY_STEPS_FN;

pub static SHUTDOWN_TX: Lazy<Mutex<Option<tokio::sync::oneshot::Sender<()>>>> =
 Lazy::new(|| Mutex::new(None));

pub static SHUTDOWN_COMPLETE_TX: Lazy<Mutex<Option<futures::channel::oneshot::Sender<()>>>> =
 Lazy::new(|| Mutex::new(None));

/// Start the tokio runtime in a new spawned thread. Returns immediately after starting it.
pub fn run(apply_steps: ApplyStepsFn) -> Result<(), Box<dyn std::error::Error>> {
 let mut rt = tokio::runtime::Runtime::new()?;

 std::thread::spawn(move || {
  rt.block_on(async move { do_run(apply_steps).await });
  std::mem::drop(rt);
  turbosql::checkpoint().unwrap();
  SHUTDOWN_COMPLETE_TX.lock().unwrap().take().unwrap().send(()).unwrap();
 });

 Ok(())
}

/// Stop the tokio runtime, kill all task threads, and checkpoint the db.
pub fn shutdown() {
 let (tx, rx) = futures::channel::oneshot::channel();
 *SHUTDOWN_COMPLETE_TX.lock().unwrap() = Some(tx);
 SHUTDOWN_TX.lock().unwrap().take().unwrap().send(()).unwrap();
 futures::executor::block_on(rx).ok();
 log::info!("shutdown complete");
}

async fn do_run(apply_steps: ApplyStepsFn) {
 *APPLY_STEPS_FN.lock().unwrap() = Some(apply_steps);
 env_logger::Builder::from_default_env().format_timestamp_millis().init();
 dbg!(env!("RUST_VERSION"));

 proc_migrations::run_proc_migrations();

 let log = warp::log("turbo_server");

 let cors = warp::cors()
  .allow_methods(&[http::Method::GET, http::Method::POST])
  .allow_headers(vec![http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
  .allow_any_origin();

 let root_node = Arc::new(schema::schema());

 let routes = (warp::path("subscriptions").and(warp::ws()).map(move |ws: warp::ws::Ws| {
  let root_node = root_node.clone();
  ws.on_upgrade(move |websocket| async move {
   juniper_warp::subscriptions::serve_graphql_ws(
    websocket,
    root_node,
    juniper_graphql_ws::ConnectionConfig::new(()),
   )
   .map(|r| {
    if let Err(e) = r {
     log::error!("Websocket error: {}", e);
    }
   })
   .await
  })
 }))
 .map(|reply| {
  // TODO#584: remove this workaround
  warp::reply::with_header(reply, "Sec-WebSocket-Protocol", "graphql-ws")
 })
 .or(warp::post().and(warp::path("graphql")).and(juniper_warp::make_graphql_filter_sync(
  schema::schema(),
  warp::any().map(move || ()).boxed(),
 )))
 .or(
  warp::get()
   .and(warp::path("playground"))
   .and(juniper_warp::playground_filter("/graphql", Some("/subscriptions"))),
 )
 .or(asset_server::asset_server())
 .with(cors)
 .with(log);

 let (tx, rx) = tokio::sync::oneshot::channel();

 let (addr, server) =
  warp::serve(routes).bind_with_graceful_shutdown(([0, 0, 0, 0], 8080), async {
   rx.await.ok();
  });

 *SHUTDOWN_TX.lock().unwrap() = Some(tx);

 // Spawn the server into a runtime
 eprintln!("Listening on {}", addr);
 tokio::task::spawn(server).await.unwrap();
}
