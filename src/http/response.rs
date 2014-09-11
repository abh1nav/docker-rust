use std::str::StrSplits;

pub struct Response {
  pub status_code: uint,
  pub body: String
}

pub fn parse(response_string: &str) -> Response {
  let status = response_string.split_str("\r\n").next().unwrap().words();
  let status_code = from_str::<uint>(status.skip(1).next().unwrap()).unwrap();
  
  let spl: StrSplits = response_string.split_str("\r\n\r\n");
  let body = spl.skip(1).next().unwrap();
  Response {
    status_code: status_code,
    body: String::from_str(body)
  }
}