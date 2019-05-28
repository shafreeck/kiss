//! pmap is a parallel map that use multiple cores

use std::collections::BTreeMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::vec;

use super::threads;
use array;
use metrohash::MetroHash;

pub type Key = Vec<u8>;

#[derive(Debug)]
pub struct Value {}

#[derive(Debug)]
pub struct Pair {
    key: Key,
    val: Value,
}

#[derive(Debug)]
pub struct ParallelMap {
    threads: threads::Threads,
    size: i64,
    slots: vec::Vec<BTreeMap<Key, Value>>,
}

impl ParallelMap {
    pub fn new(n: usize) -> ParallelMap {
        assert!(n > 0);
        let mut slots = vec::Vec::with_capacity(n);
        for i in 0..n {
            slots.push(BTreeMap::new());
        }
        ParallelMap {
            threads: threads::Threads::new(n),
            size: 0,
            slots: slots,
        }
    }

    pub fn get(&self, key: Key) -> Option<Value> {
        let mut h = MetroHash::new();
        key.hash(&mut h);
        let id = h.finish() % self.slots.len() as u64;
        println!("id is {}", id);

        self.threads.exec(id as i32, || {
            {println!("hello threads")};
        });
        None
    }
    pub fn put(key: Key, val: Value) -> i32 {
        0
    }
    pub fn delete(key: Key) -> i32 {
        0
    }
    pub fn scan(start: Key, end: Key) -> vec::Vec<Pair> {
        vec![]
    }
}
