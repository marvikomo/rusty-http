pub enum StatusCode{}
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