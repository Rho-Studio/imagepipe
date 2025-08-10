use serde::Serialize;

use std::fmt;
use std::fmt::Debug;
use std::io::Write;

type HashType = blake3::Hasher;
const HASHSIZE: usize = 32;
pub type BufHash = [u8; HASHSIZE];

#[derive(Clone)]
pub struct BufHasher {
    hash: HashType,
}
impl Default for BufHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl BufHasher {
    pub fn new() -> BufHasher {
        BufHasher {
            hash: HashType::new(),
        }
    }
    pub fn result(&self) -> BufHash {
        *self.hash.finalize().as_bytes()
    }
}
impl Debug for BufHasher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BufHasher {{ {:?} }}", self.result())
    }
}

impl Write for BufHasher {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.hash.update(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl BufHasher {
    pub fn encode_into_std_write<T>(&mut self, obj: &T)
    where
        T: Serialize,
    {
        bincode::serde::encode_into_std_write(obj, self, bincode::config::legacy()).unwrap();
    }
}
