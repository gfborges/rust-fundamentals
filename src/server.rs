use crate::http::Request;
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
                        Ok(_) => match Request::try_from(&buffer[..]) {
                            Ok(request) => {
                                println!("{:?}", request);
                                println!(
                                    "{} {} {:?}",
                                    request.method(),
                                    request.path(),
                                    request.query_string
                                );
                                if let Some(query_string) = request.query_string {
                                    println!("{:?}", query_string.get("arg1"));
                                }
                            }
                            Err(err) => println!("Invalid Request: {err}"),
                        },
                        Err(e) => {
                            println!("Falied to read from connection: {e}");
                            continue;
                        }
                    }
                }
                Err(e) => {
                    println!("ERROR: {e}");
                    continue;
                }
            };
        }
    }
}
