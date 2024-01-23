use std::fmt::{Display, Formatter, Result as ResultFmt};

#[derive(Copy, Clone, Debug)]
pub enum StatusHttp {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusHttp {
    pub fn get_cause(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl Display for StatusHttp {
    fn fmt(&self, f: &mut Formatter) -> ResultFmt {
        write!(f, "{}", *self as u16)
    }
}
