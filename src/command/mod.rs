// all sub mods
pub mod strings;

use std::collections::HashMap;
use std::vec::Vec;

use crate::context::Context;
use crate::resp;
use crate::Bytes;

#[derive(Debug)]
pub struct Command {
    pub name: Bytes,
    pub args: Vec<Bytes>,
}

#[derive(Debug)]
pub struct Constraints {}

pub struct Desc {
    pub func: Proc,
    pub cons: Constraints,
}

pub type Proc = fn(ctx: &Context) -> resp::Kind;

lazy_static! {
    pub static ref COMMANDS: HashMap<&'static str, Desc> = {
        let mut m = HashMap::new();
        m.insert(
            "get",
            Desc {
                func: strings::get,
                cons: Constraints {},
            },
        );
        m
    };
}
