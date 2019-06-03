pub mod strings;

use std::collections::HashMap;
use std::vec::Vec;

use super::Bytes;

#[derive(Debug)]
pub struct Command {
    pub name: Bytes,
    pub args: Vec<Bytes>,
}

#[derive(Debug)]
struct Constraints {
}

#[derive(Debug)]
struct Context {
    cmd: Command,
    call: Proc,
    cons: Constraints,
}

pub type Proc = fn(ctx :Context);

lazy_static! {
    static ref commands: HashMap<String, Proc> = {
        let mut m = HashMap::new();
        m
    };
}
