const MAGIC_BSDIFF: u8 = 1;

/// Compares `old` and `new` and returns a multipatch blob that can transform old -> new.
pub fn create<O: AsRef<[u8]>, N: AsRef<[u8]>>(old: O, new: N) -> anyhow::Result<Vec<u8>> {
 let old = old.as_ref();
 let new = new.as_ref();
 let mut patch = Vec::new();
 qbsdiff::Bsdiff::new(old, new).compare(std::io::Cursor::new(&mut patch))?;

 dbg!(old.len());
 dbg!(new.len());
 dbg!(patch.len());

 let patcher = qbsdiff::Bspatch::new(&patch).unwrap();
 let mut rehydrated_new = Vec::new();
 patcher.apply(old, std::io::Cursor::new(&mut rehydrated_new))?;

 assert!(dbg!(rehydrated_new == new));

 let mut v = vec![MAGIC_BSDIFF];
 v.append(&mut patch);

 Ok(v)
}

// pub fn make_bidi(<V: AsRef<[u8]>>(a: V, b: V) -> Vec<u8> {})

pub fn apply<O: AsRef<[u8]>, P: AsRef<[u8]>>(old: O, patch: P) -> anyhow::Result<Vec<u8>> {
 assert!(patch.as_ref()[0] == MAGIC_BSDIFF);
 let patcher = qbsdiff::Bspatch::new(&patch.as_ref()[1..]).unwrap();
 let mut new = Vec::new();
 patcher.apply(old.as_ref(), std::io::Cursor::new(&mut new))?;

 Ok(new)
}

#[cfg(test)]
mod tests {
 use super::*;
 #[test]
 fn has_magic() {
  let patch = create("", "").unwrap();
  assert_eq!(patch[0], MAGIC_BSDIFF);
 }
}
