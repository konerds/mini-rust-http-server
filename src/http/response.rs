use super::StatusHttp;
use std::fmt::Debug;
use std::io::{Result as ResultIo, Write};

#[derive(Debug)]
pub struct Response {
    status: StatusHttp,
    body: Option<String>,
}

impl Response {
    pub fn new(status: StatusHttp, body: Option<String>) -> Self {
        Response { status, body }
    }
    pub fn send(&self, stream: &mut impl Write) -> ResultIo<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status,
            self.status.get_cause(),
            body
        )
    }
}
