#![crate_id="docker#0.1"]
#![feature(macro_rules)]

extern crate debug;

use std::io::net::unix::UnixStream;

mod http;

fn main() {
  let socket_path = "/var/run/docker.sock";
  let request = "GET /containers/json HTTP/1.0\r\n\r\n";

  println!("Sending HTTP Request: {}", request);
  let socket = Path::new(socket_path);

  let mut stream = match UnixStream::connect(&socket) {
    Err(_) => fail!("server is not running"),
    Ok(stream) => stream,
  };

  // Send request
  match stream.write_str(request) {
    Err(_) => fail!("couldn't send request"),
    Ok(_) => {}
  };

  // Read response
  let resp = match stream.read_to_str() {
    Err(_) => fail!("response derped"),
    Ok(resp) => resp
  };

  println!("{}", resp);
}
