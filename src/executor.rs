use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::vec;

use futures::future::lazy;
use metrohash::MetroHash;
use tokio;
use tokio::prelude::*;
use tokio::sync::{mpsc, oneshot};

use crate::command::{Command, COMMANDS};
use crate::context::{ClientContext, Context, ServerContext};
use crate::resp;

// Execute a command
#[derive(Debug)]
pub struct Executor {
    n: usize,
    senders: vec::Vec<mpsc::Sender<Context>>,
}

impl Executor {
    pub fn new(n: usize) -> Executor {
        Executor {
            //db: db,
            n: n,
            senders: vec::Vec::new(),
        }
    }
    pub fn start(&mut self) {
        for _ in 0..self.n {
            let (tx, rx) = mpsc::channel(1024);
            self.senders.push(tx);

            let exec = rx.map_err(|_| ()).for_each(|ctx| {
                let f = ctx.desc.func;
                let res = f(&ctx);
                ctx.sink.send(res).unwrap();
                Ok(())
            });
            tokio::spawn(exec);
        }
    }
    pub fn execute(&self, cmd: Command) -> oneshot::Receiver<resp::Kind> {
        let (tx, rx) = oneshot::channel();
        if cmd.args.len() == 0 {
            tx.send(resp::Kind::Error("NO ARGS".as_bytes().to_vec()));
            return rx;
        }
        let mut h = MetroHash::new();
        let key = &cmd.args[0];
        key.hash(&mut h);
        let idx = h.finish() as usize % self.n;
        let sender = self.senders[idx].clone();

        let name = std::str::from_utf8(cmd.name.as_slice()).unwrap();
        let desc = COMMANDS.get(name).unwrap();

        let ctx = Context {
            cli: Arc::new(ClientContext {
                authenticated: true,
            }),
            srv: Arc::new(ServerContext {
                auth: String::from("hello"),
                clients: Vec::new(),
            }),
            cmd: Arc::new(cmd),
            desc: Arc::new(desc),
            sink: tx,
        };
        sender.send(ctx).wait().unwrap();
        rx
    }
}
