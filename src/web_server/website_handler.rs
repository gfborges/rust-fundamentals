use super::{server::Handler};
use super::http::{Request, Response, StatusCode, Method};

pub struct WebsiteHandler;


impl WebsiteHandler  {
    pub fn new() -> Self {
        Self
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&self, request: &Request) -> Response {
        match request.method() {
            Method::Get => route_handler(request),
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}

fn route_handler(request: &Request) -> Response {
    match request.path() {
        "/" => Response::new(StatusCode::Ok,Some("<h1> I AM ALIVE </h1>".to_string())),
        "/another" => Response::new(StatusCode::Ok,Some("<h1> hello </h1>".to_string())),
        _ => Response::new(StatusCode::NotFound, None)
    }
}