#![forbid(unsafe_code)]

use multihash::{Code::Identity, MultihashDigest};

pub fn random_id() -> String {
 let data: [u8; 16] = rand::random();
 bs58::encode(Identity.digest(&data).to_bytes()).into_string()
}

#[cfg(test)]
mod tests {
 #[test]
 fn it_works() {
  assert_eq!(2 + 2, 4);
  println!("{:#?}", super::random_id());
 }

 fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
  let mut rev = vec![];
  for x in xs {
   rev.insert(0, x.clone())
  }
  rev
 }

 quickcheck::quickcheck! {
  fn double_reversal_is_identity(xs: Vec<isize>) -> bool {
   xs == reverse(&reverse(&xs))
  }
 }
}
