// Turbocafe, the content-addressable file store

use multihash::{Code::Blake3_256, MultihashDigest};
use turbosql::{select, Blob, Turbosql};

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

fn hash_impl<T: AsRef<[u8]>>(content: T) -> Vec<u8> {
 Blake3_256.digest(content.as_ref()).to_bytes()
}

pub fn hash<T: AsRef<[u8]>>(content: T) -> String {
 bs58::encode(hash_impl(content)).into_string()
}

pub fn put<T: AsRef<[u8]>>(content: T) -> Result<String, TurbocafeError> {
 let hash = hash(&content);

 _Turbocafe_Entry {
  hash: Some(hash.clone()),
  content: Some(content.as_ref().to_owned()),
  ..Default::default()
 }
 .insert()?;

 Ok(hash)
}

pub fn get<T: AsRef<str>>(hash: T) -> Result<Vec<u8>, TurbocafeError> {
 Ok(
  select!(_Turbocafe_Entry "WHERE hash = ?", hash.as_ref())?
   .content
   .ok_or(TurbocafeError::DataNotAvailable)?,
 )
}

pub fn get_string<T: AsRef<str>>(hash: T) -> Result<String, TurbocafeError> {
 Ok(std::str::from_utf8(&get(hash)?)?.to_string())
}
