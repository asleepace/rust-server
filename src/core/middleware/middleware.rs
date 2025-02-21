use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub type MiddlewareResult = Result<u16, u8>;

pub trait Middleware: Send + Sync + 'static {
    fn handle(&self, context: Arc<Mutex<Context>>) -> MiddlewareResult;
}

pub struct Context<T> {
    pub request: Request,
    pub response: Option<Response>,
    pub handled: bool,
}
