use std::{str::FromStr, fmt::Display};
use std::fmt::{Result as FmtResult, Formatter};

#[derive(Debug)]
pub enum Method {
    Get,
    Delete,
    Post,
    Head,
    Put,
    Patch,
    Connect,
    Options,
    Trace
}


impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::Get),
            "DELETE" => Ok(Method::Delete),
            "POST" => Ok(Method::Post),
            "HEAD" => Ok(Method::Head),
            "PUT" => Ok(Method::Put),
            "PATCH" => Ok(Method::Patch),
            "CONNECT" => Ok(Method::Connect),
            "OPTIONS" => Ok(Method::Options),
            "TRACE" => Ok(Method::Trace),
            _ => Err(MethodError)
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Method::Get => write!(f, "{}", "GET"),
            Method::Delete => write!(f, "{}", "DELETE"),
            Method::Post => write!(f, "{}", "POST"),
            Method::Head => write!(f, "{}", "HEAD"),
            Method::Put => write!(f, "{}", "PUT"),
            Method::Patch => write!(f, "{}", "PATCH"),
            Method::Connect => write!(f, "{}", "CONNECT"),
            Method::Options => write!(f, "{}", "OPTIONS"),
            Method::Trace => write!(f, "{}", "TRACE"),
        }  
        
    }
}

pub struct MethodError;