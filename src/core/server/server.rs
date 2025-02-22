use crate::core::server::routes::RouteBuilder;
use crate::core::tcp_methods::TcpMethods;
use crate::core::ArcRwLock;
use crate::core::Request;
use crate::core::Routes;
use crate::core::ThreadSafe;

use std::cell::RefCell;
use std::io::{Error, Write};
use std::net::{TcpListener, TcpStream};
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;

use super::worker::Message;
use super::worker::Worker;

static NUM_WORKERS: usize = 4;

/// Convenience Init
pub fn create_server_on(port: u16) -> Server {
    Server::new(&format!("localhost:{}", port)).unwrap()
}

/// Server module
pub struct Server {
    listener: TcpListener,
    pub routes: Routes,
    workers: Vec<Worker>,
    receiver: Arc<Mutex<Receiver<Message>>>,
    channel: Sender<Message>,
    worker_id: RefCell<usize>,
    connections: Vec<TcpStream>,
}

impl Server {
    /// Create a new server instance with a TcpListener
    /// listening on `localhost:8080`
    pub fn new(addr: &str) -> Result<Self, std::io::Error> {
        println!("[server] binding to address: http://{}", addr);
        let listener = TcpListener::bind(addr)?;
        let routes = Routes::new();

        // Create channel for worker communication

        // Create workers
        let (sender, receiver) = std::sync::mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let workers: Vec<Worker> = (0..NUM_WORKERS).map(|id| Worker::new(id)).collect();

        Ok(Server {
            listener,
            routes,
            workers,
            receiver,
            channel: sender,
            worker_id: RefCell::new(0),
            connections: vec![],
        })
    }

    /// Start the server
    pub fn start(&mut self) {
        println!("[server] starting server...");
        loop {
            let stream = match self.listener.accept() {
                Ok((stream, _)) => stream,
                Err(e) => {
                    eprintln!("[server] error accepting connection: {:?}", e);
                    continue;
                }
            };

            match self.distribute(stream) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("[server] error distributing connection: {:?}", e);
                }
            }
        }
    }

    fn distribute(&mut self, stream: TcpStream) -> Result<(), std::io::Error> {
        println!("[server] {} {}", "-".repeat(40), "+");

        if stream.is_keep_alive() {
            println!("[server] keep-alive connection");
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "keep-alive connection",
            ));
        } else {
            println!("[server] connecting {}", stream.peer_addr().unwrap());
        }

        let mut request = Request::from(stream)?;
        let handler = match self.routes.find(&mut request) {
            Some(handler) => handler,
            None => {
                println!("[server] no route found for: {}", request.uri);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "no route found",
                ));
            }
        };

        let operation = Box::new(move || handler(&mut request));
        let worker_id = self.get_worker_id();
        self.workers[worker_id].enqueue(operation);
        Ok(())
    }

    fn get_worker_id(&self) -> usize {
        let worker_id = self.worker_id.take();
        *self.worker_id.borrow_mut() = (worker_id + 1) % NUM_WORKERS;
        worker_id
    }

    pub fn configure(&mut self, f: impl FnOnce(&mut RouteBuilder) -> ()) {
        self.routes.configure(f);
    }
}
