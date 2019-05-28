//! THREADS DESIGN
//!
//!
use std::sync::mpsc;
use std::thread;
use std::vec;

type Task = Box<FnOnce() + Send + 'static>;

#[derive(Debug)]
struct Worker {
    id: i32,
    thread: thread::JoinHandle<()>,
}
impl Worker {
    fn new(id: i32, receiver: mpsc::Receiver<Task>) -> Worker {
        let t = thread::spawn(move || {
            match receiver.recv() {
                Ok(f) => f(),
                Err(e) => {
                    return;
                }
            }
            //let f = receiver.recv().unwrap();
            //f();
        });
        Worker { id: id, thread: t }
    }
}

#[derive(Debug)]
pub struct Threads {
    senders: vec::Vec<mpsc::Sender<Task>>,
    workers: vec::Vec<Worker>,
}

impl Threads {
    pub fn new(size: usize) -> Threads {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        let mut senders = Vec::with_capacity(size);
        for id in 0..size {
            let (sender, receiver) = mpsc::channel();
            workers.push(Worker::new(id as i32, receiver));
            senders.push(sender);
        }
        Threads {
            senders: senders,
            workers: workers,
        }
    }
    pub fn exec<F>(&self, id: i32, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let sender = &self.senders[id as usize];
        println!("sender {:#?}, id {}", sender, id);
        sender.send(Box::new(f));
    }
}
