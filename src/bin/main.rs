use std::io;
use std::net;

use kiss::conf::Config;
use kiss::server::Server;

fn main() -> io::Result<()> {
    let c = Config {
        listen: "127.0.0.1:8804".to_string(),
    };
    let s = Server::new(&c);
    let lis = net::TcpListener::bind(c.listen)?;
    s.serve(lis)?;
    Ok(())
}
