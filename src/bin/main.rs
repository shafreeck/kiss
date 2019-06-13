use std::error::Error;

use kiss::conf::Config;
use kiss::server::Server;

fn main() -> Result<(), Box<dyn Error>> {
    let c = Config {
        listen: "127.0.0.1:8804".to_string(),
    };
    let mut s = Server::new(&c);
    s.listen_and_serve(c.listen)?;
    Ok(())
}
