#![forbid(unsafe_code)]

use i54_::i54;
use std::convert::TryInto;
use turbosql::Turbosql;

#[derive(thiserror::Error, Debug)]
pub enum TurbotimeError {
 #[error("TurbosqlError")]
 TurbosqlError(#[from] turbosql::Error),
}

#[allow(non_camel_case_types)]
#[derive(Turbosql, Default)]
struct _Turbotime_Entry {
 rowid: Option<i64>,
 timestamp: Option<i54>,
 instantiation_id: Option<String>,
 steps: Option<String>,
}

// pub fn put<T: AsRef<[u8]>>(content: T) -> Result<String, TurbocafeError> {
//  let hash = hash(&content);

//  _Turbocafe_Entry {
//   hash: Some(hash.clone()),
//   content: Some(content.as_ref().to_owned()),
//   ..Default::default()
//  }
//  .insert()?;

//  Ok(hash)
// }

// pub fn get<T: AsRef<str>>(hash: T) -> Result<Vec<u8>, TurbocafeError> {
//  Ok(
//   select!(_Turbocafe_Entry "WHERE hash = ?", hash.as_ref())?
//    .content
//    .ok_or(TurbocafeError::DataNotAvailable)?,
//  )
// }

// pub fn get_as_string<T: AsRef<str>>(hash: T) -> Result<String, TurbocafeError> {
//  Ok(std::str::from_utf8(&get(hash)?)?.to_string())
// }

#[cfg(test)]
mod tests {
 #[test]
 fn it_works() {
  assert_eq!(2 + 2, 4);
 }
}

pub fn now() -> i54 {
 std::time::SystemTime::now()
  .duration_since(std::time::UNIX_EPOCH)
  .unwrap()
  .as_millis()
  .try_into()
  .unwrap()
}

pub fn insert_steps(instantiation_id: String, steps: String) -> Result<(), TurbotimeError> {
 _Turbotime_Entry {
  instantiation_id: Some(instantiation_id),
  steps: Some(steps),
  timestamp: Some(now()),

  ..Default::default()
 }
 .insert()?;

 Ok(())
}
