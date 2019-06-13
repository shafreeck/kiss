#![feature(proc_macro_hygiene)]
#![feature(async_await)]
pub mod command;
pub mod conf;
pub mod error;
pub mod executor;
pub mod resp;
pub mod server;
pub mod storage;
pub mod context;

#[macro_use]
extern crate lazy_static;

use std::vec;

pub type Bytes = vec::Vec<u8>;
