
use server::Server;
mod restaurant;
mod server;

fn main() {
    restaurant::enter();
    let server = Server::new("127.0.0.1:8000".to_string());
    server.run();
}