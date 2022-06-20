use crate::web_server::thread_pool::ThreadPool;

use super::{http::{ParseError, Request, Response, StatusCode}, website_handler::WebsiteHandler};
use std::{io::Read, net::{TcpListener, TcpStream}, sync::Arc};

pub trait Handler {
    fn handle_request(&self, request: &Request) -> Response;

    fn handle_bad_request(&self, error: &ParseError) -> Response {
        println!("Failed to parse request: {error}");
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, handler: WebsiteHandler) {
        println!("listing on {}", self.addr);
        let pool = ThreadPool::new(6);
        let listener = TcpListener::bind(&self.addr).expect("port already in use");

        let the_arc_handler = Arc::new(handler);
        for stream in listener.incoming() {
            let arc_handler = Arc::clone(&the_arc_handler);
            match stream {
                Ok(stream) => {
                pool.execute(move || handle_connection(stream, arc_handler));
            },
                Err(e) => {
                    println!("Failed to connect: {}", e);
                    continue;
                }
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, handler: Arc<WebsiteHandler>) {
    let mut buffer = [0u8; 2048];
    match stream.read(&mut buffer) {
        Ok(_) => {
            let response = try_parse_request(&buffer[..], handler);
            if let Err(e) = response.send(Box::new(stream)) {
                print!("Failed to write reponse on stream: {e}");
            }
        }
        Err(e) => {
            println!("Failed to read from stream: {e}")
        }
    }
}

fn try_parse_request(buffer: &[u8], handler: Arc<WebsiteHandler>) -> Response {
    match Request::try_from(&buffer[..]) {
        Ok(request) => {
            dbg!(&request);
            handler.handle_request(&request)
        }
        Err(e) => handler.handle_bad_request(&e),
    }
}
