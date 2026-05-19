use crate::{Adler32, Adler32Hash};

impl Adler32Hash for &[u8] {
  fn hash(&self) -> u32 {
    let mut hash = Adler32::new();

    hash.write(self);
    hash.finish()
  }
}

impl Adler32Hash for &str {
  fn hash(&self) -> u32 {
    let mut hash = Adler32::new();

    hash.write(self.as_bytes());
    hash.finish()
  }
}

impl<const SIZE: usize> Adler32Hash for [u8; SIZE] {
  fn hash(&self) -> u32 {
    let mut hash = Adler32::new();

    hash.write(self);
    hash.finish()
  }
}
