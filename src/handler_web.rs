use std::fs;

use super::http::{Method, Request, Response, StatusHttp};
use super::server::HandlerRequest;

pub struct HandlerWeb {
    path_public: String,
}

impl HandlerWeb {
    pub fn new(path_public: String) -> Self {
        Self { path_public }
    }
    fn read_view(&self, name_file: &str) -> Option<String> {
        let path = format!("{}/{}", self.path_public, name_file);
        match dunce::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.path_public) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", name_file);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl HandlerRequest for HandlerWeb {
    fn handle_success(&mut self, req: &Request) -> Response {
        match req.method() {
            Method::GET => match req.path() {
                "/" => Response::new(StatusHttp::Ok, self.read_view("index.html")),
                "/hello" => Response::new(StatusHttp::Ok, self.read_view("hello.html")),
                path => match self.read_view(path) {
                    Some(contents) => Response::new(StatusHttp::Ok, Some(contents)),
                    None => Response::new(StatusHttp::NotFound, None),
                },
            },
            _ => Response::new(StatusHttp::NotFound, None),
        }
    }
}
