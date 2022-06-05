use super::StatusCode;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct Response{
    status_code: StatusCode,
    body: Option<String>
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response{
         status_code,
         body
        }
    }
}