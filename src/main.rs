#![allow(dead_code)]
#![allow(unused_variables)]

use server::Server;
use website_handler::WebsiteHandler;
mod http;
mod restaurant;
mod server;
mod threaded_talk;
mod website_handler;

fn main() {
    let server = Server::new("127.0.0.1:8000".to_string());
    let website_hanlder = WebsiteHandler::new();
    server.run(website_hanlder);
}
