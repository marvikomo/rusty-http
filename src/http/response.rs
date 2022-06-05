use super::StatusCode;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::net::TcpStream;
use std::io::{Write, Result as IoResult};

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }
    pub fn send(&self, stream: &mut TcpStream);
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let body = match &self.body {
            Some(i) => i,
            None => "",
        };
        write!(
            f,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}
