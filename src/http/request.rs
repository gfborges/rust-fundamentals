use super::Method;

pub struct Request {
    path: String,
    // express absense of value without null (typesafe)
    query_string: Option<String>,
    method: Method
}