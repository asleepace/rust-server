use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use super::{Message, Server};

pub type WorkerReceiver = Arc<Mutex<mpsc::Receiver<Message>>>;

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: WorkerReceiver, server: Arc<Server>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::Incoming(stream) => server.handle_connection(stream),
                Message::Shutdown => break,
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn join(self) -> thread::Result<()> {
        self.thread.unwrap().join()
    }
}
