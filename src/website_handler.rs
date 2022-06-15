use crate::server::Handler;
use crate::http::{Response, Request, StatusCode};

pub struct WebsiteHandler;


impl WebsiteHandler  {
    pub fn new() -> Self {
        Self
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&self, request: &Request) -> Response {
        Response::new(StatusCode::Ok,Some("<h1> I AM ALIVE </h1>".to_string()))
    }
}