use super::http::{ParseError, Request, Response, StatusCode};
use std::{io::Read, net::TcpListener};

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

    pub fn run(self, handler: impl Handler) {
        println!("listing on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).expect("port already in use");

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buffer = [0u8; 2048];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let response = self.try_parse_request(&buffer[..], &handler);
                            if let Err(e) = response.send(Box::new(stream)) {
                                print!("Failed to write reponse on stream: {e}");
                            }
                        }
                        Err(e) => {
                            println!("Failed to read from stream: {e}")
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to connect: {}", e);
                    continue;
                }
            }
        }
    }

    fn try_parse_request(&self, buffer: &[u8], handler: &impl Handler) -> Response {
        match Request::try_from(&buffer[..]) {
            Ok(request) => {
                dbg!(&request);
                handler.handle_request(&request)
            }
            Err(e) => handler.handle_bad_request(&e),
        }
    }
}
