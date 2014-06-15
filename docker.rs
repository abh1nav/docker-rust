#![crate_id="docker#0.1"]
#![feature(macro_rules)]

extern crate debug;
extern crate serialize;

mod http;
mod containers;

pub struct Docker {
  socket_path: &'static str
}

impl Docker {

  pub fn get_containers(&self) -> String {
    let method = http::GET;
    let path = "/containers/json";

    let result = http::make_request(self.socket_path, method, path);
    result
  }

}

fn main() {
  //let d = Docker { socket_path: "/var/run/docker.sock" };
  //let containers: String = d.get_containers(); 
  //println!("{}", containers);
}
