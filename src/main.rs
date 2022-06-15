#![allow(dead_code)]

use web_server::{server::Server, website_handler::WebsiteHandler};

mod restaurant;
mod threaded_talk;
mod web_server;

fn main() {
    let server = Server::new("127.0.0.1:8000".to_string());
    let website_hanlder = WebsiteHandler::new();
    server.run(website_hanlder);
}
