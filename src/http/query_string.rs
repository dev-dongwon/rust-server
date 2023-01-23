use std::collections::HashMap;

// query string case는 다양한데 다음과 같다, 배열의 경우 하나의 키에 여러값이 들어간다
// q=1&a===&g=1&g=5
#[derive(Debug)]
pub struct QueryString<'buf> {
  data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
  Single(&'buf str),
  Multiple(Vec<&'buf str>) // array
}

impl <'buf> QueryString<'buf> {
  pub fn get(&self, key: &str) -> Option<&Value> {
    self.data.get(key)
  }
}

// a=1&b=2&c&d=&e===&d=7&d=abc
impl<'buf> From<&'buf str> for QueryString<'buf> {
  fn from(s: &'buf str) -> Self {
      let mut data = HashMap::new();

      for sub_str in s.split('&') {
        // 서브 스트링 무너지 우선 구해주고
          let mut key = sub_str; // c처럼 단독인 경우도 있음
          let mut val = ""; // c처럼 값이 없는 경우도 있음
          if let Some(i) = sub_str.find('=') {
            // = 를 기준으로 왼쪽은 key, 오른쪽은 value가 되겠다
              key = &sub_str[..i];
              val = &sub_str[i + 1..];
          }

          data.entry(key)
              // existing 클로저를 선언하고 여기에 값을 임시로 담는다
              .and_modify(|existing: &mut Value| match existing {
                  // Value::Single(prev_val) => {
                  //   // 여기 들어온 시점부터는 multiple value가 됨. single은 str이니까 vec를 만들어서 이전값, 현재값을 다 담아줘야함.
                  //   let mut vec = Vec::new();
                  //   vec.push(val);
                  //   vec.push(prev_val);
                  // }
                  Value::Single(prev_val) => {
                    // 포인터를 바꿔주는 건데, 지금 existing이 바라보는 곳을 우리가 원하는 곳으로 바꿔줌. 솔직히 이거 하면서 어떻게 바로 생각하냐... 설명도 빨리 스킵해서 알아듣기도 힘드네...
                      *existing = Value::Multiple(vec![prev_val, val]);
                  }
                  // 배열이 이미 존재하면, 즉 multiple value면 그냥 push
                  Value::Multiple(vec) => vec.push(val),
              })
              // 없으면 집어넣고
              .or_insert(Value::Single(val));
      }

      QueryString { data }
  }
}