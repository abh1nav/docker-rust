//#![crate_id="docker#0.1"]
#![feature(globs,macro_rules)]

extern crate debug;
extern crate serialize;

use Containers = containers::Containers;

mod http;
pub mod containers;

pub struct Docker {
  socket_path: &'static str
}

impl Docker {

  pub fn get_containers(&self) -> Containers {
    let method = http::GET;
    let path = "/containers/json";

    let response = http::make_request(self.socket_path, method, path);
    let result = containers::parse(response.body.as_slice());
    match result {
      Err(_) => fail!("JSON response could not be decoded"),
      Ok(containers) => containers
    }
  }

}

fn main() {
  let d = Docker { socket_path: "/var/run/docker.sock" };
  let containers = d.get_containers(); 
  println!("Number of running containers = {}", containers.len());
}
