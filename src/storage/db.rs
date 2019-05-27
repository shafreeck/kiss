use std::collections::BTreeMap;

use crate::storage::pmap::*;
use crate::storage::txn::*;

#[derive(Debug)]
pub struct Db {
    m: BTreeMap<Key, Value>,
}

impl Db {
    pub fn open() -> Db {
        Db { m: BTreeMap::new() }
    }

    pub fn begin() -> Txn {
        Txn {}
    }
}
