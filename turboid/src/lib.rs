#[cfg(test)]
mod tests {
 #[test]
 fn it_works() {
  assert_eq!(2 + 2, 4);
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
