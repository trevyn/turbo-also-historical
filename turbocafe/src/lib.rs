// Turbocafe, the content-addressable file eyrie. :)

#![forbid(unsafe_code)]

use multihash::{Code::Blake3_256, MultihashDigest};
use turbosql::{execute, select, Blob, Turbosql};

#[derive(thiserror::Error, Debug)]
pub enum TurbocafeError {
 #[error("TurbosqlError")]
 TurbosqlError(#[from] turbosql::Error),
 #[error("Utf8Error")]
 Utf8Error(#[from] std::str::Utf8Error),
 #[error("Data not available for hash")]
 DataNotAvailable,
}

#[allow(non_camel_case_types)]
#[derive(Turbosql, Default)]
struct _Turbocafe_Entry {
 rowid: Option<i64>,
 hash: Option<String>,
 content: Option<Blob>,
}

fn hash_impl<U: AsRef<[u8]>>(content: U) -> Vec<u8> {
 Blake3_256.digest(content.as_ref()).to_bytes()
}

pub fn hash<U: AsRef<[u8]>>(content: U) -> String {
 bs58::encode(hash_impl(content)).into_string()
}

pub fn put_hash<U: AsRef<[u8]>>(content: U) -> Result<String, TurbocafeError> {
 let hash = hash(&content);

 _Turbocafe_Entry {
  hash: Some(hash.clone()),
  content: Some(content.as_ref().to_owned()),
  ..Default::default()
 }
 .insert()?;

 Ok(hash)
}

pub fn put_kv_new<U: AsRef<[u8]>>(content: U) -> Result<String, TurbocafeError> {
 let key = turboid::random_id();
 put_kv(&key, content)?;
 Ok(key)
}

pub fn put_kv<S: AsRef<str>, U: AsRef<[u8]>>(key: S, content: U) -> Result<(), TurbocafeError> {
 let key = key.as_ref();
 let content = content.as_ref();

 if execute!("UPDATE _turbocafe_entry SET content = ? WHERE hash = ?", content, key)? == 0 {
  _Turbocafe_Entry {
   hash: Some(key.to_owned()),
   content: Some(content.to_owned()),
   ..Default::default()
  }
  .insert()?;
 }

 Ok(())
}

pub fn get<S: AsRef<str>>(hash: S) -> Result<Vec<u8>, TurbocafeError> {
 select!(_Turbocafe_Entry "WHERE hash = ?", hash.as_ref())?
  .content
  .ok_or(TurbocafeError::DataNotAvailable)
}

pub fn get_as_string<S: AsRef<str>>(hash: S) -> Result<String, TurbocafeError> {
 Ok(std::str::from_utf8(&get(hash)?)?.to_string())
}
