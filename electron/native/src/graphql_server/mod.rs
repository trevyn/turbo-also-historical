use futures::{FutureExt as _, Stream};
use juniper::{graphql_object, graphql_subscription, FieldError, FieldResult, RootNode};
use juniper_graphql_ws::ConnectionConfig;
use juniper_warp::{playground_filter, subscriptions::serve_graphql_ws};
use std::convert::TryInto;
use std::{env, pin::Pin, sync::Arc, time::Duration};
use warp::{http, Filter};

mod datastore;
use datastore::Pdf;

mod asset_server;

#[derive(Clone)]
pub struct JuniperContext {}

impl juniper::Context for JuniperContext {}

struct Query;

#[graphql_object(context = JuniperContext)]
impl Query {
 async fn list_pdfs() -> FieldResult<Vec<Pdf>> {
  Ok(datastore::list_pdfs()?)
 }
}

struct Mutation;

#[graphql_object(context = JuniperContext)]
impl Mutation {
 async fn add_pdf(content: String) -> FieldResult<Pdf> {
  datastore::add_pdf(&content)?;
  Ok(Pdf {
   id: Some(content.len().try_into()?),
   name: Some("from addPdf".into()),
   ..Default::default()
  })
 }
}

type UsersStream = Pin<Box<dyn Stream<Item = Result<Pdf, FieldError>> + Send>>;

struct Subscription;

#[graphql_subscription(context = JuniperContext)]
impl Subscription {
 async fn users_subscription() -> UsersStream {
  let mut counter = 0;
  let stream = tokio::time::interval(Duration::from_secs(5)).map(move |_| {
   counter += 1;
   Ok(Pdf { id: Some(counter), name: Some("stream user".to_string()), ..Default::default() })
  });

  Box::pin(stream)
 }
}

type Schema = RootNode<'static, Query, Mutation, Subscription>;

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
 let qm_state = warp::any().map(move || JuniperContext {});
 let qm_graphql_filter = juniper_warp::make_graphql_filter(qm_schema, qm_state.boxed());

 let root_node = Arc::new(schema());

 eprintln!("Listening on 127.0.0.1:8080");

 let routes = (warp::path("subscriptions").and(warp::ws()).map(move |ws: warp::ws::Ws| {
  let root_node = root_node.clone();
  ws.on_upgrade(move |websocket| async move {
   serve_graphql_ws(websocket, root_node, ConnectionConfig::new(JuniperContext {}))
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
