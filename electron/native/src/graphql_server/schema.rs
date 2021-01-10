use futures::Stream;
use juniper::{graphql_object, graphql_subscription, FieldError, FieldResult};
use std::{convert::TryInto, pin::Pin, time::Duration};
use turbosql::{execute, i53, select};

mod datastore;
use datastore::Pdf;

type Schema = juniper::RootNode<'static, Query, Mutation, Subscription>;

pub fn schema() -> Schema {
 Schema::new(Query, Mutation, Subscription)
}

// @mark schema
pub struct Query;

#[fold_impls::fold_impls]
fn _query_impls() {
 #[derive(juniper::GraphQLObject, Debug)]
 struct ListPdfsResultItem {
  rowid: i53,
  name: String,
 }

 #[graphql_object]
 impl Query {
  async fn list_pdfs() -> FieldResult<Vec<ListPdfsResultItem>> {
   Ok(dbg!(select!(Vec<ListPdfsResultItem> "rowid, name FROM pdf"))?)
  }
 }

 #[derive(juniper::GraphQLObject, Debug)]
 struct ListPdfsResultItem2 {
  rowid: i53,
  name: String,
 }

 #[graphql_object]
 impl Query {
  async fn list_pdfs2() -> FieldResult<Vec<ListPdfsResultItem2>> {
   Ok(dbg!(select!(Vec<ListPdfsResultItem2> "rowid, name FROM pdf"))?)
  }
 }
}

pub struct Mutation;

#[derive(juniper::GraphQLObject, Debug)]
struct MutationResult {
 success: bool,
}

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
 async fn delete_pdf(rowid: i53) -> MutationResult {
  MutationResult { success: execute!("DELETE FROM pdf WHERE rowid = ?", rowid).is_ok() }
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
