use super::http::{Method, Request, Response, StatusHttp};
use super::server::HandlerRequest;

pub struct HandlerWeb;

impl HandlerRequest for HandlerWeb {
    fn handle_success(&mut self, req: &Request) -> Response {
        match req.method() {
            Method::GET => match req.path() {
                "/" => Response::new(StatusHttp::Ok, Some("<h1>Welcome</h1>".to_string())),
                "/hello" => {
                    Response::new(StatusHttp::Ok, Some("<h1>Hello World!</h1>".to_string()))
                }
                _ => Response::new(StatusHttp::NotFound, None),
            },
            _ => Response::new(StatusHttp::NotFound, None),
        }
    }
}
