use futures::FutureExt as _;
use std::sync::Arc;
use warp::{http, Filter};

mod asset_server;
mod proc_migrations;
mod schema;

pub use schema::rust_log;

pub use schema::ApplyStepsFn;
use schema::APPLY_STEPS_FN;

// @mark server
#[tokio::main]
pub async fn run(apply_steps: ApplyStepsFn) {
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

 eprintln!("Listening on 127.0.0.1:8080");

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
 .or(
  warp::post()
   .and(warp::path("graphql"))
   .and(juniper_warp::make_graphql_filter(schema::schema(), warp::any().map(move || ()).boxed())),
 )
 .or(
  warp::get()
   .and(warp::path("playground"))
   .and(juniper_warp::playground_filter("/graphql", Some("/subscriptions"))),
 )
 .or(asset_server::asset_server())
 .with(cors)
 .with(log);

 warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
