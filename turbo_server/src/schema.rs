use d_macro::*;
use futures::Stream;
use i54_::i54;
use juniper::{graphql_object, graphql_subscription, FieldError, FieldResult};
use once_cell::sync::Lazy;
use std::time::Duration;
use std::{convert::TryInto, future::Future, pin::Pin, sync::Mutex};
use tokio::task::spawn_blocking;
use turbosql::{execute, select, Blob, Turbosql};

pub type ApplyStepsFn =
 Box<dyn Fn(String, String) -> Pin<Box<dyn Future<Output = String> + Send + Sync>> + Send + Sync>;

pub static APPLY_STEPS_FN: Lazy<Mutex<Option<ApplyStepsFn>>> = Lazy::new(|| Mutex::new(None));

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
 pub created_time: Option<i54>,
}

pub fn rust_log(entry: String) {
 Log { entry: Some(entry), created_time: Some(turbotime::now()), ..Default::default() }
  .insert()
  .unwrap();
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

  async fn get(key: String) -> FieldResult<String> {
   Ok(turbocafe::get_as_string(key)?)
  }
 }

 #[graphql_object]
 impl Query {
  async fn list_cards_full() -> FieldResult<Vec<Card>> {
   Ok(
    spawn_blocking(move || {
     select!(Vec<Card>
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
     )
    })
    .await??,
   )
  }
 }
}

pub struct Mutation;

#[graphql_object]
impl Mutation {
 async fn add_blank_card() -> FieldResult<Card> {
  let now = turbotime::now();
  let new_hash = d!(#? turbocafe::put_hash(
   r#"{"doc":{"type":"doc","content":[{"type":"paragraph"}]},"selection":{"type":"text","anchor":1,"head":1}}"#
  ))?;

  let mut card = Card {
   instantiation_id: Some(new_hash),
   created_time: Some(now),
   modified_time: Some(now),
   content: Some(turbocafe::put_kv_new("")?),
   answer: Some(turbocafe::put_kv_new("")?),
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

 async fn put_kv(key: Option<String>, value: String) -> FieldResult<String> {
  let key = key.unwrap_or_else(turboid::random_id);
  turbocafe::put_kv(&key, value)?;
  Ok(key)
 }

 async fn recv_steps(instantiation_id: String, steps: String) -> FieldResult<String> {
  // let old_content =
  //  turbocafe::get_string(select!(Card "WHERE rowid = ?", rowid).unwrap().instantiation_id.unwrap())
  //   .unwrap();

  turbocafe::put_hash(
   r#"{"doc":{"type":"doc","content":[{"type":"paragraph"}]},"selection":{"type":"text","anchor":1,"head":1}}"#,
  )?;

  let old_content = turbocafe::get_as_string(&instantiation_id).unwrap();

  let new_content = prosemirror_collab_server::apply_steps(&old_content, &steps)?; // no-op for now

  let fut = (*APPLY_STEPS_FN.lock().unwrap()).as_ref().unwrap()(old_content, steps.clone());
  d!(fut.await);

  turbotime::insert_steps(instantiation_id, steps)?;

  d!(&new_content);

  // let patch = multipatch::create(&old_content, &new_content).unwrap();
  // let rehydrated_new = multipatch::apply(&old_content, &patch).unwrap();
  // assert!(dbg!(rehydrated_new == <String as AsRef<[u8]>>::as_ref(&new_content)));

  // dbg!(patch.len());

  // let old_hash = dbg!(turbocafe::hash(&old_content));
  // let new_hash = dbg!(turbocafe::hash(&new_content));
  // let _patch_hash = dbg!(turbocafe::hash(&patch));

  // dbg!(turbocafe::get_string(&old_hash)).ok();
  // dbg!(turbocafe::get_string(&new_hash)).ok();
  // dbg!(turbocafe::get(&patch_hash)).ok();

  // dbg!(turbocafe::put(&old_content)).ok();
  // let new_hash = dbg!(turbocafe::put(&new_content))?;
  // dbg!(turbocafe::put(&patch)).ok();

  // dbg!(turbocafe::get_string(&old_hash)).ok();
  // dbg!(turbocafe::get_string(&new_hash)).ok();
  // dbg!(turbocafe::get(patch_hash)).ok();

  // let now = turbotime::now();

  // execute!(
  //  "
  //   UPDATE card SET
  //    instantiation_id = ?,
  //    modified_time = ?
  //    WHERE rowid = ?
  //  ",
  //  new_hash,
  //  now,
  //  rowid
  // )?;

  Ok("".to_string())

  // return prosemirror instead

  // Ok(select!(Card "WHERE rowid = ?", rowid)?)
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
