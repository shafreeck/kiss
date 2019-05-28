use std::collections::BTreeMap;
use std::vec;

use super::pmap::ParallelMap;
use super::txn::Txn;

#[derive(Debug)]
pub struct Db {
    m: ParallelMap,
}

impl Db {
    pub fn open() -> Db {
        Db {
            m: ParallelMap::new(100),
        }
    }
    pub fn get(&self, key: vec::Vec<u8>) {
        self.m.get(key);
    }
}
