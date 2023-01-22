use std::collections::HashMap;

// query string case는 다양한데 다음과 같다, 배열의 경우 하나의 키에 여러값이 들어간다
// q=1&a===&g=1&g=5
pub struct QueryString<'buf> {
  data: HashMap<&'buf str, Value<'buf>>,
}

pub enum Value<'buf> {
  Single(&'buf str),
  Multiple(Vec<&'buf str>) // array
}

impl <'buf> QueryString<'buf> {
  pub fn get(&self, key: &str) -> Option<&Value> {
    self.data.get(key)
  }
}
