use std::io;

use kiss::conf::Config;
use kiss::server::Server;

fn main() -> io::Result<()> {
    let c = Config {
        listen: "127.0.0.1:8804".to_string(),
    };
    let s = Server::new(&c);
    s.listen_and_serve(c.listen)?;
    //    std::thread::sleep(std::time::Duration::from_secs(5));
    Ok(())
}
