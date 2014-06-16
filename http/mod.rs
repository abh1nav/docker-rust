use std::io::net::unix::UnixStream;
use std::string::String;

pub mod response;

///
/// A macro to concatenate non-literal Strings.
///
macro_rules! cat(
  ($inp:ident, $($sp:ident),+) => ({
    $($inp.push_str($sp);)+
  });
)

pub enum RequestType {
  GET,
  PUT,
  POST,
  DELETE
}

fn make_request_str(request_type: RequestType, path: &str) -> String {
  let mut result: String = String::new();
  let suffix = " HTTP/1.0\r\n\r\n";

  result.push_str(match request_type {
    GET => "GET ",
    PUT => "PUT ",
    POST => "POST ",
    DELETE => "DELETE "
  });

  cat!(result, path, suffix);
  result.to_string()
}

pub fn make_request(socket_path: &str, request_type: RequestType, path: &str) -> response::Response {
  let http_request = make_request_str(request_type, path);
  let socket = Path::new(socket_path);

  let mut stream = match UnixStream::connect(&socket) {
    Err(_) => fail!("server is not running"),
    Ok(stream) => stream,
  };

  // Send request
  match stream.write_str(http_request.as_slice()) {
    Err(_) => fail!("couldn't send request"),
    Ok(_) => {}
  };

  // Read response
  let resp: String = match stream.read_to_str() {
    Err(_) => fail!("response derped"),
    Ok(resp) => resp
  };

  response::parse(resp.as_slice())
}

#[test]
fn test_make_request_str() {
  let request_type = GET;
  let result = make_request_str(request_type, "/hello/world");
  let expected = "GET /hello/world HTTP/1.0\r\n\r\n".to_string();
  assert!(result == expected);
}