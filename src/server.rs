use crate::http::{Request, Response, StatusCode};
use std::{io::Read, net::TcpListener};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self) {
        println!("listing on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("OK {}", addr);
                    let mut buffer = [0u8; 2048];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(StatusCode::Ok)
                                }
                                Err(_) => Response::new(StatusCode::BadRequest),
                            };
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
                    println!("Failed to connect: {e}");
                    continue;
                }
            };
        }
    }
}
