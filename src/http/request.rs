use std::str::Utf8Error;
use super::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{ Display, Formatter, Result as fmtResult, Debug };
use std::str;

pub struct Request {
    path: String,
    query_string: Option<String>, // none or Some<T>
    method: Method,
}


impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;
        unimplemented!();
    }
}

// option은 말 그대로 optionable한 value 지칭
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    unimplemented!();
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}