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

    // &mut dyn Write => dynamic dispatch 라는 개념이 있구나! 를 알아야됨
    // 런타임 시점에 Write이 정확히 뭔지 알 수 있고, 컴파일러가 자체적으로 가지고 있는 레코드에 기록된 곳에서 호출함 => 최적화 없는 대신 바이너리 코드가 줄어들겠지?

    // impl Write => static dispatch
    // Write은 다양한 경우의 수가 있을텐데, 모든 경우의 수에 대해 미리 compile 해놓은 후, 런타임때 그냥 가져오기만함
    // 속도가 당연히 빠름, 근데 바이너리 파일 크기가 좀 늘어나겠지?
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
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