use std::{fmt::{Formatter, Display, Result as FmtResult}, str::Utf8Error};

use std::str;
use super::{Method, method::MethodError};

pub struct Request<'buffer> {
    path: &'buffer str,
    // express absense of value without null (typesafe)
    query_string: Option<&'buffer str>,
    method: Method
}

impl<'buffer> Request<'buffer> {
    pub fn path(&'buffer self) -> &'buffer str {
        self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }
    pub fn query_string(&'buffer self) -> Option<&'buffer str> {
        self.query_string
    }
}

impl<'buffer> TryFrom<&'buffer [u8]> for Request<'buffer> {
    type Error = ParseError;

    fn try_from(buffer: &'buffer [u8]) -> Result<Self, Self::Error> {
        let request  = str::from_utf8(buffer)?;

        let (method,request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol == "HTTP 1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method :Method = method.parse()?;

        let mut query_string: Option<&str> = None;
        if let Some(i) = path.find('?') {
            query_string = Some(&path[i+1..]);
            path = &path[..i];
        }
        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i+1..]))
        } 
    }
    None
}

pub enum ParseError {
    InvalidEncoding,
    InvalidRequest,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}