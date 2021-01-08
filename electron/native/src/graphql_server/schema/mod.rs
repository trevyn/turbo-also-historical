use futures::Stream;
use juniper::{graphql_object, graphql_subscription, FieldError, FieldResult};
use std::{convert::TryInto, pin::Pin, time::Duration};

mod datastore;
use datastore::Pdf;

type Schema = juniper::RootNode<'static, Query, Mutation, Subscription>;

pub fn schema() -> Schema {
 Schema::new(Query, Mutation, Subscription)
}

pub struct Query;

#[graphql_object]
impl Query {
 async fn list_pdfs() -> FieldResult<Vec<Pdf>> {
  Ok(datastore::list_pdfs()?)
 }
}

pub struct Mutation;

#[graphql_object]
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

pub struct Subscription;

#[graphql_subscription]
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
