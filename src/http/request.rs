use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

use super::method::Method;
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request {
    fn from_bytes_array(buf: &[u8]) -> Result<Self, String>{
       unimplemented!()
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error>{
        // match str::from_utf8(buf){
        //     Ok(request) => {},
        //     Err(_) => return Err(ParseError::InvalidEncoding)
        // }
        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {}
        //     Err(e) => return Err(e)
        // }
        let request = str::from_utf8(buf)?; //overide the utf8error by impl From trait
        unimplemented!()
    }


}

pub enum ParseError{
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod
}
impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
       write!(f, "{}", self.message())
    }
}

impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
       write!(f, "{}", self.message())
    }
}

impl ParseError{
    fn message(&self)->&str{
        match self {
            Self::InvalidProtocol => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method"
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
       Self::InvalidEncoding
    }
}

impl Error for ParseError {

}


