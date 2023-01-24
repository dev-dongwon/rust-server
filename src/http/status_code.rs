use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode {
  // 어렵다. 근데 찬찬히 생각해보자
  // 스택에 할당되는 rust primitive들은 모두 copy를 구현함.
  // 근데 heap에 할당되는 String 같은 것들은 copy를 구현하지 않음. 주소값만 저장되니까.
  // u16로 형변환을 하고 싶을 때, 이 형변환은 어떻게 해야 하는가? self에 해당하는 저 enum이 u16, 즉 스택에 있는 데이터를 copy 해야됨.
  // 이걸 derive로 선언해주는거.
  // 근데 copy만 선언하면 또 안되네? copy를 구현하려면 반드시 clone도 선언해줘야됨.
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}