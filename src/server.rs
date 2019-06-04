use std::error::Error;
use std::fmt::Display;
use std::sync::{Arc, Mutex};
use std::vec;

use tokio::codec::Framed;
use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

use super::command::Command;
use super::conf::Config;
use super::error;
use super::resp;
use super::storage::db::Db;
use super::Bytes;

pub struct Server {
    db: Db,
}

impl Server {
    pub fn new(c: &Config) -> Server {
        println!("new server {:?}", c);
        Server { db: Db::open() }
    }
    pub fn serve(&self, lis: TcpListener) -> Result<(), Box<dyn Error>> {
        let incoming = lis.incoming();
        let server = incoming
            .map_err(|e| eprintln!("accept failed = {:?}", e))
            .for_each(move |sock| {
                println!("connected {:?}", sock);
                let transport = Framed::new(sock, resp::Codec::new());
                let (tx, rx) = transport.split();
                let task = tx
                    .send_all(rx.and_then(|req| match parse_command(req) {
                        Ok(cmd) => {
                            println!("cmd {:?}", String::from_utf8(cmd.name.clone()));
                            Ok(resp::Kind::SimpleString(cmd.name))
                        }
                        Err(err) => Ok(resp::Kind::Error(err.description().as_bytes().to_vec())),
                    }))
                    .then(|resp| {
                        if let Err(err) = resp {
                            println!("send response failed:{:?}", err);
                        }
                        Ok(())
                    });

                tokio::spawn(task);
                Ok(())
            });
        tokio::run(server);
        Ok(())
    }

    pub fn listen_and_serve(&self, addr: String) -> Result<(), Box<dyn Error>> {
        let sock_addr = addr.parse().expect("parse listen address failed");
        let lis = TcpListener::bind(&sock_addr).expect("unable to bind TCP listener");
        self.serve(lis)
    }
}

pub fn parse_command(k: resp::Kind) -> error::Result<Command> {
    match k {
        resp::Kind::Array(a) => {
            let parsed: Result<vec::Vec<Bytes>, error::RedisError> = a
                .elems
                .into_iter()
                .map(|field| match field {
                    resp::Kind::BulkString(Some(s)) => Ok(s),
                    _ => Err(error::RedisError::new("protocol error")),
                })
                .collect();

            let parsed = parsed?;
            Ok(Command {
                name: parsed[0].clone(),
                args: parsed[1..].to_vec(),
            })
        }
        resp::Kind::Inline(s) => {
            let parsed: vec::Vec<Bytes> = s
                .split(|c| c.is_ascii_whitespace())
                .map(|field| vec::Vec::from(field))
                .into_iter()
                .collect();

            Ok(Command {
                name: parsed[0].clone(),
                args: parsed[1..].to_vec(),
            })
        }
        _ => Err(error::RedisError::new("protocol invalid")),
    }
}
