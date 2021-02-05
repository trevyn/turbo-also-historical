use futures::Stream;
use i54_::i54;
use juniper::{graphql_object, graphql_subscription, FieldError, FieldResult};
use std::convert::TryInto;
use std::pin::Pin;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use turbosql::{execute, select, Turbosql};

#[derive(juniper::GraphQLObject, Turbosql, Default, Debug)]
pub struct Card {
 // Remember: you can mark these as deprecated at any time! (or maybe delete them entirely? Is the schema a semi-hidden implementation detail?)
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
 pub presentation_order: Option<i54>,
}

#[derive(Turbosql, Default, Debug)]
pub struct CardList {
 pub rowid: Option<i54>,
 pub list: Option<String>,
 pub created_time: Option<f64>,
}

#[derive(Turbosql, Default, Debug)]
pub struct Log {
 pub rowid: Option<i54>,
 pub entry: Option<String>,
 pub created_time: Option<f64>,
}

pub fn rust_log(entry: String) {
 Log { entry: Some(entry), ..Default::default() }.insert().unwrap();
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
   Ok(select!(Vec<ShortCard> "rowid, name FROM card")?)
  }
 }

 #[graphql_object]
 impl Query {
  async fn list_cards_full() -> FieldResult<Vec<Card>> {
   Ok(select!(Vec<Card>
    "
     WITH split(word, str) AS (
      SELECT '', list FROM cardlist
      UNION ALL SELECT
       substr(str, 0, instr(str, ',')),
       substr(str, instr(str, ',') + 1)
       FROM split WHERE str != ''
     )
     SELECT DISTINCT card.*
      FROM split
      LEFT JOIN card ON card.rowid = word
      WHERE word != '' AND card.rowid IS NOT NULL
    "
   )?)
  }
 }
}

pub struct Mutation;

#[graphql_object]
impl Mutation {
 async fn add_card(content: String, answer: String) -> FieldResult<Card> {
  let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64();
  let card = Card {
   content: Some(content.to_string()),
   answer: Some(answer),
   name: Some(format!("a card of {} bytes", content.len())),
   filesize: Some(content.len().try_into()?),
   created_time: Some(now),
   modified_time: Some(now),
   ..Default::default()
  };

  let rowid = card.insert()?;

  execute!(
   "
    REPLACE INTO cardlist(rowid, list)
     VALUES(1, ? || ',' || (SELECT list FROM cardlist))
   ",
   rowid
  )?;

  Ok(card)
 }

 async fn update_card(rowid: i54, content: String, answer: String) -> FieldResult<Card> {
  let new_content = content;
  let old_content = select!(Card "WHERE rowid = ?", rowid).unwrap().content.unwrap();

  let mut patch = Vec::new();
  qbsdiff::Bsdiff::new(old_content.as_bytes(), new_content.as_bytes())
   .compare(std::io::Cursor::new(&mut patch))
   .unwrap();

  dbg!(old_content.len());
  dbg!(new_content.len());
  dbg!(patch.len());

  let old_hash = dbg!(turbocafe::hash(&old_content));
  let new_hash = dbg!(turbocafe::hash(&new_content));
  let patch_hash = dbg!(turbocafe::hash(&patch));

  dbg!(turbocafe::get_string(&old_hash)).ok();
  dbg!(turbocafe::get_string(&new_hash)).ok();
  // dbg!(turbocafe::get(&patch_hash)).ok();

  dbg!(turbocafe::put(&old_content)).ok();
  dbg!(turbocafe::put(&new_content)).ok();
  dbg!(turbocafe::put(&patch)).ok();

  dbg!(turbocafe::get_string(old_hash)).ok();
  dbg!(turbocafe::get_string(new_hash)).ok();
  // dbg!(turbocafe::get(patch_hash)).ok();

  let patcher = qbsdiff::Bspatch::new(&patch).unwrap();
  let mut target = Vec::new();
  patcher.apply(old_content.as_bytes(), std::io::Cursor::new(&mut target))?;

  assert!(dbg!(target == new_content.as_bytes()));

  let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64();
  execute!(
   "
    UPDATE card SET
     content = ?,
     answer = ?,
     modified_time = ?
     WHERE rowid = ?
   ",
   new_content,
   answer,
   now,
   rowid
  )?;
  Ok(select!(Card "WHERE rowid = ?", rowid)?)
 }

 async fn delete_card(rowid: i54) -> FieldResult<bool> {
  execute!("DELETE FROM card WHERE rowid = ?", rowid)?;
  Ok(true)
 }

 async fn shuffle_cards() -> FieldResult<bool> {
  execute!(
   "
    REPLACE INTO cardlist(rowid, list)
     VALUES(1, (
      SELECT group_concat(rowid) || ','
       FROM (SELECT rowid FROM card ORDER BY random())
     ))
   "
  )?;
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
