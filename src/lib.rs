#![feature(proc_macro_hygiene)]
#![feature(async_await)]
pub mod conf;
pub mod resp;
pub mod server;
pub mod storage;
pub mod command;
pub mod error;

#[macro_use]
extern crate lazy_static;


use std::vec;

pub type Bytes = vec::Vec<u8>;