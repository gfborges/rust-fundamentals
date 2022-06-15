pub enum StatusCode {
    Ok = 200,

}
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}


impl Response {
    pub fn new(status_code: StatusCode) -> Self {
        Self {
            status_code: StatusCode::Ok,
            body: Some(String::from("<html><body>Hello World</body></html>"))
        }
    }
}