#[cfg(test)]
use std::io::Command;
#[cfg(test)]
use std::io::timer;

use serialize::{json, Decodable};
use serialize::json::DecoderError;

use Containers = super::common::containers::Containers;
use http = super::http;

pub struct Docker {
  socket_path: String
}

impl Docker {

  fn parse_get_containers(json_string: &str) -> Result<Containers, DecoderError> {
    let json_object = json::from_str(json_string);
    let mut decoder = json::Decoder::new(json_object.unwrap());
    Decodable::decode(&mut decoder)
  }

  pub fn get_containers(&self) -> Containers {
    let method = http::GET;
    let path = "/containers/json?all=1&size=1";

    let response = http::make_request(self.socket_path.as_slice(), method, path);
    if response.status_code >= 200 && response.status_code < 300 {
      let result = Docker::parse_get_containers(response.body.as_slice());
      match result {
        Err(_) => fail!("JSON response could not be decoded"),
        Ok(containers) => containers
      }  
    }
    else {
      fail!("HTTP response code was {}", response.status_code);
    }
  }

  pub fn stop_container(&self, id: &str) {
    let method = http::POST;
    let mut path = String::new();
    path.push_str("/containers/");
    path.push_str(id);
    path.push_str("/stop");

    let response = http::make_request(self.socket_path.as_slice(), method, path.as_slice());
    if response.status_code < 200 || response.status_code >= 300 {
      fail!("HTTP response code was {}", response.status_code);
    }
  }

  pub fn remove_container(&self, id: &str) {
    let method = http::DELETE;
    let mut path = String::new();
    path.push_str("/containers/");
    path.push_str(id);
    path.push_str("?v=1");

    let response = http::make_request(self.socket_path.as_slice(), method, path.as_slice());
    if response.status_code < 200 || response.status_code >= 300 {
      fail!("HTTP response code was {}", response.status_code);
    }
  }

}

///
/// Test(s)
///

#[allow(type_limits)]
#[test]
fn test_get_containers() {
  let client = Docker { socket_path: "/var/run/docker.sock".to_string() };
  let containers = client.get_containers();
  let count: uint = containers.len();
  assert!(count >= 0);
}

#[test]
fn test_stop_and_remove_container() {
  // First start a container
  let output = match Command::new("docker")
                          .arg("run")
                          .arg("-d")
                          .arg("busybox")
                          .output() {
    Ok(process_output) => String::from_utf8(process_output.output).unwrap(),
    Err(e) => fail!("Failed to start the container we wanted to stop: {}", e)
  };

  let replace = output.as_slice().replace("\r\n", "");
  let container_id = replace.as_slice().trim();
  timer::sleep(3000);

  // Stop the container
  let client = Docker { socket_path: "/var/run/docker.sock".to_string() };
  client.stop_container(container_id.as_slice());

  // Remove the container
  client.remove_container(container_id.as_slice());
}