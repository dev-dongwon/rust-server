use std::io::{Result as IoResult, Write};

use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    // dynamic dispatch 라는 개념이 있구나! 를 알아야됨
    // 함수가 Write 특성을 구현하는 타입을 반환한다는 것만 알면 되고, 특별히 어떤 타입이 반환될지에 대해서는 알 필요 없음
    // 이런 방식은 유연성을 확보할 수 있음
    pub fn send(&self, stream: &mut dyn Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}