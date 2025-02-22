use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

static KEEP_ALIVE: &[u8] = b"HTTP/2.0 200 OK\r\nContent-Length: 0\r\n\r\n";

pub trait TcpMethods {
    fn is_connected(&self) -> bool;
    fn is_keep_alive(&self) -> bool;
    fn send_keep_alive(&mut self) -> std::io::Result<()>;
}

impl TcpMethods for TcpStream {
    fn is_connected(&self) -> bool {
        self.peer_addr().is_ok()
    }

    fn is_keep_alive(&self) -> bool {
        match self.peek(&mut [0u8; 8]) {
            Ok(bytes_read) => bytes_read == 0,
            Err(_) => false,
        }
    }

    fn send_keep_alive(&mut self) -> std::io::Result<()> {
        println!("[tcp_methods] sending keep_alive!");
        self.write_all(KEEP_ALIVE)?;
        self.flush()?;
        Ok(())
    }
}
