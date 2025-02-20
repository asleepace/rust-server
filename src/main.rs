use crate::core::*;

mod core;

fn main() {
    let server = server::create_server_on(8080);

    server.configure(|route| {
        route.def("GET", "/", get_home_page);
        route.def("GET", "*", get_home_page);
    });

    server.start();
}

// example definitions
fn get_home_page(request: &mut Request) -> http::Response {
    println!("[main] serving route: /");
    request.send(http::static_file("src/public/index.html"))
}
