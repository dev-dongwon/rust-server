use server::Server;
use website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;

fn main() {
    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run(WebsiteHandler);
}