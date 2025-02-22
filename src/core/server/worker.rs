use std::io::Error;
use std::net::TcpStream;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use crate::core::http::http_codec::HttpResponse;
use crate::core::{ArcRwLock, Request, ThreadSafe};

use super::server::Server;
use super::{RouteHandler, Routes};

pub type Operation = Box<dyn FnOnce() -> Result<u8, Error> + Sync + Send + 'static>;

pub enum Message {
    Handle(Arc<Mutex<Vec<Operation>>>),
    Shutdown,
}

pub type WorkerReceiver = Arc<Mutex<mpsc::Receiver<Message>>>;

pub struct Worker {
    id: usize,
    sender: mpsc::Sender<Message>,
    thread: Option<thread::JoinHandle<()>>,
    operations: Arc<Mutex<Vec<Operation>>>,
}

impl Worker {
    pub fn new(id: usize) -> Worker {
        let (sender, receiver) = mpsc::channel();

        Worker {
            id,
            sender,
            operations: Arc::new(Mutex::new(Vec::new())),
            thread: Some(thread::spawn(move || loop {
                let message = match receiver.recv() {
                    Ok(message) => message,
                    Err(e) => {
                        eprintln!("[worker] #{} error receiving message: {:?}", id, e);
                        break;
                    }
                };

                match message {
                    Message::Shutdown => break,
                    Message::Handle(operations) => match operations.try_lock() {
                        Ok(mut operations) => {
                            while let Some(operation) = operations.pop() {
                                if let Err(e) = operation() {
                                    eprintln!("[worker] #{} error handling operation: {:?}", id, e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("[worker] #{} error locking operations: {:?}", id, e);
                        }
                    },
                }
            })),
        }
    }

    pub fn enqueue(&mut self, operation: Operation) {
        self.operations.lock().unwrap().push(operation);
        match self.sender.send(Message::Handle(self.operations.clone())) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("[worker] #{} error sending message: {:?}", self.id, e);
            }
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn join(self) -> thread::Result<()> {
        self.thread.unwrap().join()
    }
}
