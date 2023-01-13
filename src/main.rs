fn main() {
    let server = server::Server::new(String::from("127.0.0.1:8080"));
    server.run();
}

 mod server {
     pub struct Server {
         addr: String,
     }
     
     impl Server {
         pub fn new(addr: String) -> Self {
             Self {
                 addr
             }
         }
     
         pub fn run(self) {
             println!("Listening on {}", self.addr);
         }
     }

 }

struct Request {
    path: String,
    query_string: Option<String>, // none or Some<T>
    method: Method,
}

enum Method {
    GET,
    POST,
    DELETE,
    PUT,
    PATCH,
    OPTIONS,
    HEAD,
    TRACE
}
