// Turbocafe, the content-addressable file store

use multihash::Code::Blake3_256;
use multihash::MultihashDigest;

fn hash_impl<T: AsRef<[u8]>>(content: T) -> Vec<u8> {
 Blake3_256.digest(content.as_ref()).to_bytes()
}

pub fn hash<T: AsRef<[u8]>>(content: T) -> String {
 bs58::encode(hash_impl(content)).into_string()
}

pub fn put<T: AsRef<[u8]>>(content: T) -> String {
 hash(content)
}

pub fn get<T: AsRef<str>>(_hash: T) -> Option<Vec<u8>> {
 None
}
