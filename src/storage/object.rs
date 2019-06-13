use std::collections::{BTreeMap, BTreeSet, LinkedList};

use super::super::Bytes;

#[derive(Debug)]
pub enum Object {
    Value(Bytes),
    List(LinkedList<Bytes>),
    Set(BTreeSet<Bytes>),
    ZSet((BTreeMap<f64, &'static BTreeSet<Bytes>>, BTreeSet<Bytes>)),
    Dict(BTreeMap<Bytes, Object>),
}

pub type Dict = BTreeMap<Bytes, Object>;