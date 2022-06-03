use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

use super::method::{Method, MethodError};
use super::{QueryString, QueryStringValue};
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<&'buf str>,
    method: Method,
}

// impl Request<'buf> {
//     fn from_bytes_array(buf: &'buf [u8]) -> Result<Self, String> {
//         unimplemented!()
//     }
// }

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    //GET /search?name=marv&sort=1 HTTP/1.1
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        // match str::from_utf8(buf){
        //     Ok(request) => {},
        //     Err(_) => return Err(ParseError::InvalidEncoding)
        // }
        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {}
        //     Err(e) => return Err(e)
        // }
        let request = str::from_utf8(buf)?; //overide the utf8error by impl From trait
                                            // match get_next_word(request){
                                            //     Some((method, request)) => {},
                                            //     None => return Err(ParseError::InvalidRequest)
                                            // }

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?; //converts option to result
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        let method: Method = "hey".parse()?;
        let mut query_string = None;
        // match path.find('?'){
        //     Some(i)=>{
        //         query_string = Some(&path[i + 1..]);
        //         path = &path[..i]
        //     }
        //     None => {}
        // }
        if let Some(i) = path.find('?'){  //used if let instead of match
            query_string = Some(&path[i + 1..]);
            path = &path[..i];
        }
       Ok(Self {
           path: path,
           query_string,
           method
       })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, val) in request.chars().enumerate() {
        if val == ' ' || val == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    unimplemented!()
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
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

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Error for ParseError {}
