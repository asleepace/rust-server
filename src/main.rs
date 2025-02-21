use crate::core::*;
use std::{fs::File, io};
mod core;

/// --- MAIN ---

fn main() {
    let mut server = server::create_server_on(8080);

    server.configure(|route| {
        // route.def("GET", "/", get_home_page);
        route.def("GET", "*", get_catch_all);
        // route.def("GET", "/log", |req| {
        //     req.send(http::static_file("/log.html"))
        // });
        // route.def("GET", "/events", |req| {
        //     req.send(http::static_file("/events.html"))
        // });
    });

    server.start();
}

// example definitions
fn get_home_page(request: &mut Request) -> http::Response {
    println!("[main] serving route: /");
    // request.send(http::static_file("/index.html"))
    let mut file_handle = File::open("src/public/index.html")?;
    io::copy(&mut file_handle, &mut request.stream())?;
    request.close()?;
    Ok(200)
}

// example catch-all route
fn get_catch_all(request: &mut Request) -> http::Response {
    let static_file = util::find_static_file(request.uri());
    util::copy_static_file(request, static_file)?;
    Ok(200)
}
