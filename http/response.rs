use std::str::StrSplits;

pub struct Response {
  pub body: String
}

pub fn parse(response_string: &str) -> Response {
  let spl: StrSplits = response_string.split_str("\r\n\r\n");
  let body = spl.skip(1).next().unwrap();
  Response {
    body: String::from_str(body)
  }
}