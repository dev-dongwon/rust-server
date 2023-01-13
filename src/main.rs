use http::Request;
use http::method;
use server::Server;

mod server;
mod http;

fn main() {
    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run();
}