use futures::Stream;
use i54_::i54;
use juniper::{graphql_object, graphql_subscription, FieldError, FieldResult};
use std::convert::TryInto;
use std::pin::Pin;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use turbosql::{execute, select, Blob, Turbosql};

#[derive(juniper::GraphQLObject, Turbosql, Default, Debug)]
pub struct Card {
 // Remember: you can mark these as deprecated at any time! (or maybe delete them entirely? Is the schema a semi-hidden implementation detail?)
 pub rowid: Option<i54>,
 pub content: Option<String>, // deprecated
 pub answer: Option<String>,  // deprecated
 pub component_id: Option<String>,
 pub instantiation_id: Option<String>, // currently just the multihash of content, as returned by turbocafe
 pub created_time: Option<i54>,
 pub modified_time: Option<i54>,
 pub last_display_time: Option<i54>,
 pub next_display_time: Option<i54>,
}

#[allow(non_camel_case_types)]
#[derive(juniper::GraphQLObject, Turbosql, Default, Debug)]
pub struct Component_Instantiation_Data {
 pub rowid: Option<i54>,
 pub component_id: Option<String>,
 pub instantiation_id: Option<String>,
 pub data_string_1: Option<String>,
 #[graphql(skip)]
 pub data_blob_1: Option<Blob>,
 pub data_i54_1: Option<i54>,
 pub data_i54_2: Option<i54>,
}

#[derive(Turbosql, Default, Debug)]
pub struct CardList {
 pub rowid: Option<i54>,
 pub list: Option<String>,
 pub created_time: Option<i64>,
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
 }

 #[graphql_object]
 impl Query {
  async fn list_cards_short() -> FieldResult<Vec<ShortCard>> {
   Ok(select!(Vec<ShortCard> "rowid FROM card")?)
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
 async fn add_blank_card() -> FieldResult<Card> {
  let now: i54 = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis().try_into()?;
  let new_hash = dbg!(turbocafe::put("i'm a new card from turbocafe!"))?;

  let mut card = Card {
   instantiation_id: Some(new_hash),
   created_time: Some(now),
   modified_time: Some(now),
   ..Default::default()
  };

  card.rowid = Some(card.insert()?.try_into()?);

  // add to top of current cardlist

  execute!(
   "
    REPLACE INTO cardlist(rowid, list)
     VALUES(1, ? || ',' || (SELECT list FROM cardlist))
   ",
   card.rowid
  )?;

  Ok(card)
 }

 async fn update_card(rowid: i54, content: String, answer: String) -> FieldResult<Card> {
  let new_content = dbg!(content);
  let old_content = select!(Card "WHERE rowid = ?", rowid).unwrap().content.unwrap();

  dbg!(prosemirror_collab_server::apply_steps(old_content.clone(), new_content.clone()))?; // no-op for now

  let patch = multipatch::create(&old_content, &new_content).unwrap();
  let rehydrated_new = multipatch::apply(&old_content, &patch).unwrap();
  assert!(dbg!(rehydrated_new == <String as AsRef<[u8]>>::as_ref(&new_content)));

  dbg!(patch.len());

  // let old_hash = dbg!(turbocafe::hash(&old_content));
  // let new_hash = dbg!(turbocafe::hash(&new_content));
  // let _patch_hash = dbg!(turbocafe::hash(&patch));

  // dbg!(turbocafe::get_string(&old_hash)).ok();
  // dbg!(turbocafe::get_string(&new_hash)).ok();
  // dbg!(turbocafe::get(&patch_hash)).ok();

  // dbg!(turbocafe::put(&old_content)).ok();
  let new_hash = dbg!(turbocafe::put(&new_content))?;
  // dbg!(turbocafe::put(&patch)).ok();

  // dbg!(turbocafe::get_string(&old_hash)).ok();
  // dbg!(turbocafe::get_string(&new_hash)).ok();
  // dbg!(turbocafe::get(patch_hash)).ok();

  let now: i54 = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis().try_into()?;

  execute!(
   "
    UPDATE card SET
     content = ?,
     answer = ?,
     instantiation_id = ?,
     modified_time = ?
     WHERE rowid = ?
   ",
   new_content,
   answer,
   new_hash,
   now,
   rowid
  )?;

  // return prosemirror instead

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
   Ok(Card { rowid: Some(counter.into()), ..Default::default() })
  });

  Box::pin(stream)
 }
}
