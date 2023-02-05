use crate::http::{ Request, Response, StatusCode, response, ParseError };
use std::convert::TryFrom;
use std::net::TcpListener;
use std::io::{Write, Read};

pub trait Handler {
  fn handle_request(&mut self, request: &Request) -> Response;
  // default로 트레잇에 직접 선언 가능, 물론 override도 가능.
  fn handle_bad_request(&mut self, e: &ParseError) -> Response {
    println!("Failed to parse request: {}", e);
    Response::new(StatusCode::BadRequest, None)
}
}

pub struct Server {
    addr: String,
}

fn arr(a: [u8; 5]) {}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);
      
        let _listener: TcpListener = TcpListener::bind(self.addr).unwrap();

        loop {
          match _listener.accept() {
            Ok((mut stream, _)) => {
              let mut buffer = [0; 1024];
              match stream.read(&mut buffer) {
                Ok(_) => {
                  //from_utf8_lossy: Converts a slice of bytes to a string, including invalid characters. 에러 안나고 세이프하게!
                  println!("Received a request:{}", String::from_utf8_lossy(&buffer));

                  // 구현한 tryFrom에 위 스트림 버퍼를 넣고 정상동작이면 패스, 에러면 에러 메시지를 프린트
                  let response = match Request::try_from(&buffer[..]) {
                    Ok(request) => handler.handle_request(&request),
                    Err(e) => handler.handle_bad_request(&e),
                  };

                  if let Err(e) = response.send(&mut stream) {
                    println!("Failed to send response: {}", e)
                  }
                }
                Err(e) => println!("Failed to read from connection: {}", e),
              }
            },
            Err(e) => println!("Failed to establish connection: {}", e)
          }

          let res: Result<(std::net::TcpStream, std::net::SocketAddr), std::io::Error> = _listener.accept();

          if res.is_err() {
            continue;
          }

          let (stream, addr) = res.unwrap();
        }
    }
}
