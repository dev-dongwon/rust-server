use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::net::TcpListener;
use std::io::Read;

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

    pub fn run(self) {
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
                  match Request::try_from(&buffer[..]) {
                    Ok(request) => {},
                    Err(e) => println!("Failed to parse Request: {}", e)
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
