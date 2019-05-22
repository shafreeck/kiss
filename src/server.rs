use std::io;
use std::net::{TcpListener, TcpStream};

use crate::conf::Config;

pub struct Server {
}

impl Server {
    pub fn new(c :&Config)-> Server{
        println!("new server {:?}", c);
        Server{}
    }
    pub fn serve(&self, lis :TcpListener )->io::Result<()>{
        for stream in lis.incoming() {
            self.handle_client(stream.unwrap());
        }
        Ok(())
    }

    fn handle_client(&self, stream :TcpStream){
            println!("{:?}", stream);
    }
}
