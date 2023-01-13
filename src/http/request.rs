use super::Method;

pub struct Request {
    path: String,
    query_string: Option<String>, // none or Some<T>
    method: Method,
}
