use futures::FutureExt as _;
use juniper_graphql_ws::ConnectionConfig;
use juniper_warp::{playground_filter, subscriptions::serve_graphql_ws};
use std::{env, sync::Arc};
use warp::{http, Filter};

mod api;
use api::{Mutation, Query, Subscription};

mod asset_server;

type Schema = juniper::RootNode<'static, Query, Mutation, Subscription>;

fn schema() -> Schema {
 Schema::new(Query, Mutation, Subscription)
}

// @mark server
#[tokio::main]
pub async fn run() {
 env::set_var("RUST_LOG", "warp_subscriptions");
 env_logger::init();

 let log = warp::log("warp_subscriptions");

 let cors = warp::cors()
  .allow_methods(&[http::Method::GET, http::Method::POST])
  .allow_headers(vec![http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
  .allow_any_origin();

 let qm_schema = schema();
 let qm_graphql_filter =
  juniper_warp::make_graphql_filter(qm_schema, warp::any().map(move || ()).boxed());

 let root_node = Arc::new(schema());

 eprintln!("Listening on 127.0.0.1:8080");

 let routes = (warp::path("subscriptions").and(warp::ws()).map(move |ws: warp::ws::Ws| {
  let root_node = root_node.clone();
  ws.on_upgrade(move |websocket| async move {
   serve_graphql_ws(websocket, root_node, ConnectionConfig::new(()))
    .map(|r| {
     if let Err(e) = r {
      eprintln!("Websocket error: {}", e);
     }
    })
    .await
  })
 }))
 .map(|reply| {
  // TODO#584: remove this workaround
  warp::reply::with_header(reply, "Sec-WebSocket-Protocol", "graphql-ws") // transport-
 })
 .or(warp::post().and(warp::path("graphql")).and(qm_graphql_filter))
 .or(
  warp::get()
   .and(warp::path("playground"))
   .and(playground_filter("/graphql", Some("/subscriptions"))),
 )
 .or(asset_server::asset_server())
 .with(cors)
 .with(log);

 warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
