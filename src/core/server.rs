use crate::core::ThreadSafe;
use std::io::Error;
use std::net::{TcpListener, TcpStream};

use super::request::Request;
use super::routes::RouteBuilder;
use super::{ArcRwLock, Routes};

/// Convenience Init
///
/// Will panic if the server cannot be created
pub fn create_server_on(port: u16) -> Server {
    Server::new(&format!("localhost:{}", port)).unwrap()
}

/// Server module
pub struct Server {
    listener: TcpListener,
    pub routes: ThreadSafe<Routes>,
}

impl Server {
    /// Create a new server instance with a TcpListener
    /// listening on `localhost:8080`
    pub fn new(addr: &str) -> Result<Self, Error> {
        println!("[server] binding to address: http://{}", addr);
        let listener = TcpListener::bind(addr)?;
        let routes = ThreadSafe::new(Routes::new());
        Ok(Server { listener, routes })
    }

    /// Start the server
    pub fn start(&mut self) {
        println!("[server] starting server...");
        for stream in self.listener.incoming() {
            match stream {
                Err(error) => panic!("[server] error: {}", error),
                Ok(stream) => self.handle_connection(stream),
            }
        }
    }

    /// handle connection
    pub fn handle_connection(&self, stream: TcpStream) {
        println!("[server] connecting {}", stream.peer_addr().unwrap());
        let mut request = match Request::from(stream) {
            Err(error) => panic!("[server] error: {}", error),
            Ok(request) => request,
        };

        let handler = match self.routes.read(|routes| routes.find(&mut request)) {
            Some(handler) => handler,
            None => {
                println!("[server] no route found for: {}", request.uri);
                return;
            }
        };

        match handler(&mut request) {
            Ok(_) => {
                // request.close()
            }
            Err(error) => panic!("[server] route handler error: {}", error),
        };
    }

    pub fn configure(&self, f: impl FnOnce(&mut RouteBuilder) -> ()) {
        self.routes.write(|routes| routes.configure(f));
    }
}
