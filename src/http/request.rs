use super::method::Method;
use super::Query;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as ResultFmt};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query: Option<Query<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn method(&self) -> &Method {
        &self.method
    }
    pub fn query(&self) -> Option<&Query> {
        self.query.as_ref()
    }
}

fn get_token_next(req: &str) -> Option<(&str, &str)> {
    for (i, c) in req.chars().enumerate() {
        if c.is_whitespace() || c == '\r' {
            return Some((&req[..i], &req[i + 1..]));
        }
    }
    None
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ErrorParseRequest;
    fn try_from(buffer: &'buf [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buffer)?;
        let (method, request) = get_token_next(request).ok_or(ErrorParseRequest::Request)?;
        let (mut path, request) = get_token_next(request).ok_or(ErrorParseRequest::Request)?;
        let (protocol, _) = get_token_next(request).ok_or(ErrorParseRequest::Request)?;
        if protocol != "HTTP/1.1" {
            return Err(ErrorParseRequest::Protocol);
        }
        let method: Method = method.parse()?;
        let mut query = None;
        if let Some(i) = path.find('?') {
            query = Some(Query::from(&path[i + 1..]));
            path = &path[..i];
        }
        Ok(Self {
            path,
            query,
            method,
        })
    }
}

pub enum ErrorParseRequest {
    Request,
    Encoding,
    Protocol,
    Method,
}

impl ErrorParseRequest {
    fn message(&self) -> &str {
        match self {
            Self::Request => "Invalid Request",
            Self::Encoding => "Invalid Encoding",
            Self::Protocol => "Invalid Protocol",
            Self::Method => "Invalid Method",
        }
    }
}

impl From<Utf8Error> for ErrorParseRequest {
    fn from(_: Utf8Error) -> Self {
        Self::Encoding
    }
}

impl Display for ErrorParseRequest {
    fn fmt(&self, f: &mut Formatter) -> ResultFmt {
        write!(f, "{}", self.message())
    }
}

impl Debug for ErrorParseRequest {
    fn fmt(&self, f: &mut Formatter) -> ResultFmt {
        write!(f, "{}", self.message())
    }
}

impl Error for ErrorParseRequest {}
