use std::net::TcpListener;
use std::io::{Read, Write};
use crate::http::{Request,Response, StatusCode};
use std::convert::TryFrom;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn run(self) {
        println!("listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop{
            match listener.accept(){
                Ok((mut stream, _)) =>{
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer){
                        Ok(_) =>{
                            println!("Received a request:{}", String::from_utf8_lossy(&buffer));
                            //Request::try_from(&buffer as &[u8]); //we need to convert the buffer array to bytes slice
                            let response = match Request::try_from(&buffer[..]) {// another way to do this
                                Ok(request) => {
                                    //println!("Received a request {}" )
                                    dbg!(request);
                                     Response::new(StatusCode::Ok, Some("<h1>It works!!</h1>".to_string()))
                                   
                                }
                                Err(e) => {
                                    println!("Failed to parse a request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                 }
                            };

                            if let Err(e) = response.send(&mut stream){
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read connection: {}", e)
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e)
            }
            let res = listener.accept();
            if res.is_err(){
                continue;
            }
            let sream = res.unwrap();
        }
    }
}
