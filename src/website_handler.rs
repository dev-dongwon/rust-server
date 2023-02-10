use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;
use std::fs;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        // canonicalize: Returns the canonical, absolute form of a path with all intermediate components normalized and symbolic links resolved.
        // 간단히 말해 절대 경로 반환. -> 지금 만든 서버의 경우, 상대경로로 하면 root로 접근할수 있어서 보안 문제 있음.
        match fs::canonicalize(path) {
            Ok(path) => {
              // public path로 시작하는게 정상
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                  // 아니면 보여주지 않음
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}