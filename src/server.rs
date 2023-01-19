use std::net::TcpListener;

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
      
        let _listener: TcpListener = TcpListener::bind(self.addr).unwrap();

        loop {
          match _listener.accept() {
            Ok((stream, _)) => {
              println!("ok");
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
