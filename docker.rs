#[cfg(test)]
use std::io::Command;
#[cfg(test)]
use std::io::timer;

use serialize::{json, Decodable};
use serialize::json::DecoderError;

use Containers = super::common::containers::Containers;
use Version = super::common::version::Version;
use SysInfo = super::common::sys_info::SysInfo;
use http = super::http;

pub struct Docker {
  pub socket_path: String
}

impl Docker {

  ///
  /// GET /containers/json
  ///

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

  ///
  /// POST /containers/(id)/stop
  ///

  pub fn stop_container(&self, id: &str) {
    self.stop_container_impl(id, None);
  }

  pub fn stop_container_with_timeout(&self, id: &str, wait_time: uint) {
    self.stop_container_impl(id, Some(wait_time));
  }

  fn stop_container_impl(&self, id: &str, wait_time: Option<uint>) {
    let method = http::POST;
    let mut path = String::new();
    path.push_str("/containers/");
    path.push_str(id);
    path.push_str("/stop");

    match wait_time {
      Some(timeout_value) => {
        // If a wait time was specified, include it in the query string
        path.push_str("?t=");
        path.push_str(timeout_value.to_str().as_slice());
      }
      None => {
        // Don't do anything
      }
    };

    let response = http::make_request(self.socket_path.as_slice(), method, path.as_slice());
    if response.status_code < 200 || response.status_code >= 300 {
      fail!("HTTP response code was {}", response.status_code);
    }
  }

  ///
  /// POST /containers/(id)/restart
  ///

  pub fn restart_container(&self, id: &str) {
    self.restart_container_impl(id, None);
  }

  pub fn restart_container_with_timeout(&self, id: &str, wait_time: uint) {
    self.restart_container_impl(id, Some(wait_time));
  }

  fn restart_container_impl(&self, id: &str, wait_time: Option<uint>) {
    let method = http::POST;
    let mut path = String::new();
    path.push_str("/containers/");
    path.push_str(id);
    path.push_str("/restart");

    match wait_time {
      Some(timeout_value) => {
        // If a wait time was specified, include it in the query string
        path.push_str("?t=");
        path.push_str(timeout_value.to_str().as_slice());
      }
      None => {
        // Don't do anything
      }
    };

    let response = http::make_request(self.socket_path.as_slice(), method, path.as_slice());
    if response.status_code < 200 || response.status_code >= 300 {
      fail!("HTTP response code was {}", response.status_code);
    }
  }


  ///
  /// DELETE /containers/(id)/
  ///

  pub fn remove_container(&self, id: &str) {
    self.remove_container_impl(id, false);
  }

  pub fn remove_container_with_force(&self, id:&str) {
    self.remove_container_impl(id, true); 
  }

  fn remove_container_impl(&self, id:&str, force: bool) {
    let method = http::DELETE;
    let mut path = String::new();
    path.push_str("/containers/");
    path.push_str(id);
    path.push_str("?v=1");

    if force {
      path.push_str("&f=1");
    }

    let response = http::make_request(self.socket_path.as_slice(), method, path.as_slice());
    if response.status_code < 200 || response.status_code >= 300 {
      fail!("HTTP response code was {}", response.status_code);
    }
  }

  ///
  /// GET /info
  ///

  fn parse_get_sys_info(json_string: &str) -> Result<SysInfo, DecoderError> {
    let json_object = json::from_str(json_string);
    let mut decoder = json::Decoder::new(json_object.unwrap());
    Decodable::decode(&mut decoder)
  }

  pub fn get_sys_info(&self) -> SysInfo {
    let method = http::GET;
    let path = "/info";
    let response = http::make_request(self.socket_path.as_slice(), method, path);
    if response.status_code == 200 {
      let result = Docker::parse_get_sys_info(response.body.as_slice());
      match result {
        Err(_) => fail!("JSON response could not be decoded"),
        Ok(sys_info) => sys_info
      }  
    }
    else {
      fail!("HTTP response code was {}", response.status_code);
    }

  }

  ///
  /// GET /version
  ///

  fn parse_get_version(json_string: &str) -> Result<Version, DecoderError> {
    let json_object = json::from_str(json_string);
    let mut decoder = json::Decoder::new(json_object.unwrap());
    Decodable::decode(&mut decoder)
  }

  pub fn get_version(&self) -> Version {
    let method = http::GET;
    let path = "/version";
    let response = http::make_request(self.socket_path.as_slice(), method, path);
    if response.status_code == 200 {
      let result = Docker::parse_get_version(response.body.as_slice());
      match result {
        Err(_) => fail!("JSON response could not be decoded"),
        Ok(version) => version
      }  
    }
    else {
      fail!("HTTP response code was {}", response.status_code);
    }

  }

}

///
/// Test(s)
///

#[cfg(test)]
fn make_client() -> Docker {
  Docker { socket_path: "/var/run/docker.sock".to_string() }
}

#[cfg(test)]
fn start_busybox_container() -> Option<String> {
  match Command::new("docker").arg("run").arg("-d")
                              .arg("busybox").output() {
    Ok(process_output) => {
      let output = String::from_utf8(process_output.output).unwrap();
      timer::sleep(1000);
      let clean_output = output.as_slice().replace("\r\n", "");
      let container_id = clean_output.as_slice().trim();
      Some(String::from_str(container_id))
    }
    Err(_) => None
  }
}

#[allow(type_limits)]
#[test]
fn test_get_containers() {
  let client = make_client();
  let containers = client.get_containers();
  let count: uint = containers.len();
  assert!(count >= 0);
}

#[test]
fn test_stop_and_remove_container() {
  // Start a test container
  let container_id = match start_busybox_container() {
    Some(id) => id,
    None => fail!("Failed to start test container")
  };

  // Stop test container
  let client = Docker { socket_path: "/var/run/docker.sock".to_string() };
  client.stop_container(container_id.as_slice());

  // Remove test container
  client.remove_container(container_id.as_slice());
}

#[test]
fn test_restart_container() {
  let container_id = match start_busybox_container() {
    Some(id) => id,
    None => fail!("Failed to start test container")
  };

  let client = make_client();
  client.restart_container(container_id.as_slice());
  timer::sleep(3000);

  client.stop_container(container_id.as_slice());
  client.remove_container(container_id.as_slice());
}

#[test]
fn test_get_sys_info() {
  let client = make_client();
  let sys_info: SysInfo = client.get_sys_info();
  assert!(sys_info.Debug == 0);
  assert!(sys_info.DriverStatus.len() > 0);
}

#[test]
fn test_get_version() {
  let client = make_client();
  let version = client.get_version();
  assert!(version.Version != "".to_string());
  assert!(version.GitCommit != "".to_string());
  assert!(version.GoVersion != "".to_string());
}
