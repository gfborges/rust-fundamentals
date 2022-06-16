use std::env;
use std::fs::read_to_string;

use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new() -> Self {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        Self { public_path }
    }

    pub fn public_path(&self) -> &String {
        &self.public_path
    }

    fn read_file(&self, path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, path);
        read_to_string(path).ok()
    }

    fn route_handler(&self, request: &Request) -> Response {
        if let Some(body) = self.read_file(request.path()) {
            Response::new(StatusCode::Ok, Some(body))
        } else if request.path() == "/" {
            Response::new(StatusCode::Ok, self.read_file("index.html"))
        } else{
            Response::new(StatusCode::NotFound, None)
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&self, request: &Request) -> Response {
        match request.method() {
            Method::Get => self.route_handler(request),
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
