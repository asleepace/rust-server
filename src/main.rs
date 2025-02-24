use crate::core::*;
use std::{fs::File, io};
mod core;

/// --- MAIN ---

fn main() {
    let mut server = server::create_server_on(8080);

    server.configure(|route| {
        route.def("GET", "/log", |req| req.send_static("log.html"));
        route.def("GET", "/events", |req| req.send_static("events.html"));
        route.def("GET", "*", get_catch_all);
    });

    server.start();
}

// example catch-all route
fn get_catch_all(request: &mut Request) -> http::Response {
    let static_file = util::find_static_file(request.uri());
    util::copy_static_file(request, static_file)?;
    request.close()?;
    Ok(200)
}
