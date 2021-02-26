use super::schema::Card;
use turbosql::{execute, select, Blob, Turbosql};

#[cfg(not(doctest))]
pub fn run_proc_migrations() {
 log::info!("running proc migrations");
 execute!("BEGIN EXCLUSIVE TRANSACTION").unwrap();

 // direct content & answer -> turbocafe kv

 select!(Vec<Card> "WHERE substr(content,1,1) = '<'").unwrap().iter().for_each(|c| {
  let id = turbocafe::put_kv_new(&c.content.as_ref().unwrap()).unwrap();
  execute!("UPDATE card SET content = ? WHERE rowid = ?", id, c.rowid).unwrap();
 });

 select!(Vec<Card> "WHERE substr(answer,1,1) = '<'").unwrap().iter().for_each(|c| {
  let id = turbocafe::put_kv_new(&c.answer.as_ref().unwrap()).unwrap();
  execute!("UPDATE card SET answer = ? WHERE rowid = ?", id, c.rowid).unwrap();
 });

 execute!("COMMIT").unwrap();
 log::info!("proc migrations complete");
}
