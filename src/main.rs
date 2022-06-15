#![allow(dead_code)]
#![allow(unused_variables)]

use server::Server;
mod http;
mod restaurant;
mod server;
mod threaded_talk;

fn main() {
    let server = Server::new("127.0.0.1:8000".to_string()); // -+
    server.run();
}
