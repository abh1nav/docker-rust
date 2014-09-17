#[cfg(test)]
use std::io::Command;
#[cfg(test)]
use std::io::timer;
#[cfg(test)]
use std::time::Duration;

use super::common::containers::Containers as Containers;
use super::common::images::Images as Images;
use super::common::version::Version as Version;
use super::common::sys_info::SysInfo as SysInfo;

use super::methods::container as container;
use super::methods::image as image;
use super::methods::info as info;

pub struct Docker {
  pub socket_path: String
}

impl Docker {

  ///
  /// GET /containers/json
  ///

  pub fn get_containers(&self) -> Containers {
    container::get_containers(self.socket_path.as_slice())
  }

  ///
  /// POST /containers/(id)/stop
  ///

  pub fn stop_container(&self, id: &str) {
    container::stop_container_impl(self.socket_path.as_slice(), id, None);
  }

  pub fn stop_container_with_timeout(&self, id: &str, wait_time: uint) {
    container::stop_container_impl(self.socket_path.as_slice(), id, Some(wait_time));
  }

  ///
  /// POST /containers/(id)/restart
  ///

  pub fn restart_container(&self, id: &str) {
    container::restart_container_impl(self.socket_path.as_slice(), id, None);
  }

  pub fn restart_container_with_timeout(&self, id: &str, wait_time: uint) {
    container::restart_container_impl(self.socket_path.as_slice(), id, Some(wait_time));
  }

  ///
  /// DELETE /containers/(id)/
  ///

  pub fn remove_container(&self, id: &str) {
    container::remove_container_impl(self.socket_path.as_slice(), id, false);
  }

  pub fn remove_container_with_force(&self, id:&str) {
    container::remove_container_impl(self.socket_path.as_slice(), id, true); 
  }

  ///
  /// GET /images/json
  ///

  pub fn get_images(&self) -> Images {
    image::get_images(self.socket_path.as_slice())
  }

  ///
  /// GET /info
  ///

  pub fn get_sys_info(&self) -> SysInfo {
    info::get_sys_info(self.socket_path.as_slice())
  }

  ///
  /// GET /version
  ///

  pub fn get_version(&self) -> Version {
    info::get_version(self.socket_path.as_slice())
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
  match Command::new("docker").arg("run").arg("-t").arg("-d")
                              .arg("busybox:latest").output() {
    Ok(process_output) => {
      let output = String::from_utf8(process_output.output).unwrap();
      timer::sleep(Duration::milliseconds(1000));
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
  timer::sleep(Duration::milliseconds(3000));

  client.stop_container(container_id.as_slice());
  client.remove_container(container_id.as_slice());
}

#[allow(type_limits)]
#[test]
fn test_get_images() {
  let client = make_client();
  let images = client.get_images();
  let count: uint = images.len();
  assert!(count >= 0);
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
