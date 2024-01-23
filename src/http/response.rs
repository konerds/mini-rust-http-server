use super::StatusHttp;

#[derive(Debug)]
pub struct Response {
    status: StatusHttp,
    body: Option<String>,
}

impl Response {
    pub fn new(status: StatusHttp, body: Option<String>) -> Self {
        Response { status, body }
    }
}
