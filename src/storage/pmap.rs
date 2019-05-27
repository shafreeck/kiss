//! pmap is a parallel map that use multiple cores

use std::collections::BTreeMap;
use std::vec;

use array;

pub type Key = Vec<u8>;

#[derive(Debug)]
pub struct Value {}

#[derive(Debug)]
pub struct Pair {
    key: Key,
    val: Value,
}

pub struct ParallelMap {
    size: i64,
    mask: i32,
    slots: [BTreeMap<Key, Value>; 256],
}

impl ParallelMap {
    pub fn new(n: i32) -> ParallelMap {
        let mut n = n;
        if n > 256 {
            n = 256;
        }
        let m = ParallelMap {
            size: 0,
            mask: n,
            slots: array::init!([BTreeMap::new(); 256]),
        };
        m
    }

    pub fn get(key: Key) -> Option<Value> {
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
