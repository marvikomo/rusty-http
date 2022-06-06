use super::server::Handler;
use super::http::{Request, Response, StatusCode, Method};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
       match request.method(){
           Method::GET => match request.path(){
               "/" => Response::new(StatusCode::Ok, Some("<h1>Welcome</h1>".to_string())),
               _ => Response::new(StatusCode::NotFound, None)
           }
           _ => Response::new(StatusCode::NotFound, None)
       }
    }
}

