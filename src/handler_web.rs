use super::http::{Request, Response, StatusHttp};
use super::server::HandlerRequest;

pub struct HandlerWeb;

impl HandlerRequest for HandlerWeb {
    fn handle_success(&mut self, req: &Request) -> Response {
        dbg!(req);
        Response::new(StatusHttp::Ok, Some("<h1>Hello World!!!!</h1>".to_string()))
    }
}
