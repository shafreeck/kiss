use std::io;
use std::net::{TcpListener, TcpStream};

use crate::conf::Config;
use crate::storage::Db;

pub struct Server {
    db: Db,
}

impl Server {
    pub fn new(c: &Config) -> Server {
        println!("new server {:?}", c);
        Server { db: Db::open() }
    }
    pub fn serve(&self, lis: TcpListener) -> io::Result<()> {
        println!("{:?}", self.db);
        for stream in lis.incoming() {
            self.handle_client(stream.unwrap());
        }
        Ok(())
    }

    fn handle_client(&self, stream: TcpStream) {
        println!("{:?}", stream);
    }
}
