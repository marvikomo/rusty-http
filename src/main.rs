fn main() {
    let get = Method::GET;
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;

    let server = Server::new("127.0.0.1:8080".to_string());
     server.run();
}

struct Server {
    addr: String,

}

impl Server {
    fn new(addr: String) -> Self {
        Self{
            addr
        }
    }
    fn run(self){
        println!("listening on {}", self.addr);
    }
}

struct Request {
    path: String,
    query_string: Option<String>,
    method: String
}

enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}