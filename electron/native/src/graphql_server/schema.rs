use futures::Stream;
use i54_::i54;
use juniper::{graphql_object, graphql_subscription, FieldError, FieldResult};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{convert::TryInto, pin::Pin};
use turbosql::{execute, select};

#[derive(juniper::GraphQLObject, turbosql::Turbosql, Default, Debug)]
pub struct Card {
 pub rowid: Option<i54>,
 pub id: Option<i32>,
 pub filesize: Option<i54>,
 pub name: Option<String>,
 pub content: Option<String>,
 pub answer: Option<String>,
 pub created_time: Option<f64>,
 pub modified_time: Option<f64>,
 pub last_display_time: Option<f64>,
 pub next_display_time: Option<f64>,
}

type Schema = juniper::RootNode<'static, Query, Mutation, Subscription>;

pub fn schema() -> Schema {
 Schema::new(Query, Mutation, Subscription)
}

// @mark schema
pub struct Query;

#[fold_impls::fold_impls]
fn _query_impls() {
 #[derive(juniper::GraphQLObject, Debug)]
 struct ShortCard {
  rowid: i54,
  name: String,
 }

 #[graphql_object]
 impl Query {
  async fn list_cards_short() -> FieldResult<Vec<ShortCard>> {
   Ok(dbg!(select!(Vec<ShortCard> "rowid, name FROM card"))?)
  }
 }

 #[graphql_object]
 impl Query {
  async fn list_cards_full() -> FieldResult<Vec<Card>> {
   Ok(dbg!(select!(Vec<Card> "ORDER BY rowid DESC"))?)
  }
 }
}

pub struct Mutation;

#[graphql_object]
impl Mutation {
 async fn add_card(content: String) -> FieldResult<Card> {
  let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64();
  let card = Card {
   content: Some(content.to_string()),
   name: Some(format!("a card of {} bytes", content.len())),
   filesize: Some(content.len().try_into()?),
   created_time: Some(now),
   modified_time: Some(now),
   ..Default::default()
  };

  card.insert()?;

  Ok(card)
 }

 async fn update_card(rowid: i54, content: String) -> FieldResult<Card> {
  let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64();
  execute!("UPDATE card SET content = ?, modified_time = ? WHERE rowid = ?", content, now, rowid)?;
  Ok(select!(Card "WHERE rowid = ?", rowid)?)
 }

 async fn delete_card(rowid: i54) -> FieldResult<bool> {
  execute!("DELETE FROM card WHERE rowid = ?", rowid)?;
  Ok(true)
 }
}

type CardStream = Pin<Box<dyn Stream<Item = Result<Card, FieldError>> + Send>>;

pub struct Subscription;

#[graphql_subscription]
impl Subscription {
 async fn card_stream() -> CardStream {
  let mut counter = 0;
  let stream = tokio::time::interval(Duration::from_secs(5)).map(move |_| {
   counter += 1;
   Ok(Card { id: Some(counter), name: Some("stream user".to_string()), ..Default::default() })
  });

  Box::pin(stream)
 }
}
