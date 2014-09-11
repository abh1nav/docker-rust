#![allow(non_snake_case)]

#[cfg(test)]
use serialize::{json, Decodable};

#[deriving(Decodable)]
pub struct SysInfo {
  pub Containers: uint,
  pub Debug: uint,
  pub Driver: String,
  pub DriverStatus: Vec<Vec<String>>,
  pub ExecutionDriver: String,
  pub IPv4Forwarding: uint,
  pub Images: uint,
  pub IndexServerAddress: String,
  pub InitPath: String,
  pub InitSha1: String,
  pub KernelVersion: String,
  pub MemoryLimit: uint,
  pub NEventsListener: uint,
  pub NFd: uint,
  pub NGoroutines: uint,
  pub SwapLimit: uint
}

#[test]
fn test_sys_info_deserialization() {
  let json_string = "{\"Containers\":1,\"Debug\":0,\"Driver\":\"devicemapper\",\"DriverStatus\":[[\"Pool Name\",\"docker-8:1-1065722-pool\"],[\"Data file\",\"/var/lib/docker/devicemapper/devicemapper/data\"],[\"Metadata file\",\"/var/lib/docker/devicemapper/devicemapper/metadata\"],[\"Data Space Used\",\"3170.3 Mb\"],[\"Data Space Total\",\"102400.0 Mb\"],[\"Metadata Space Used\",\"4.4 Mb\"],[\"Metadata Space Total\",\"2048.0 Mb\"]],\"ExecutionDriver\":\"native-0.2\",\"IPv4Forwarding\":1,\"Images\":91,\"IndexServerAddress\":\"https://index.docker.io/v1/\",\"InitPath\":\"/usr/bin/docker\",\"InitSha1\":\"\",\"KernelVersion\":\"3.9.0\",\"MemoryLimit\":0,\"NEventsListener\":0,\"NFd\":16,\"NGoroutines\":14,\"SwapLimit\":0}"; 
  let json_object = json::from_str(json_string);
  let mut decoder = json::Decoder::new(json_object.unwrap());
  let s: SysInfo = Decodable::decode(&mut decoder).unwrap();

  assert!(s.Containers == 1);
  assert!(s.Images == 91);
  assert!(s.Debug == 0);
  assert!(s.DriverStatus.len() == 7);
  assert!(s.InitPath == "/usr/bin/docker".to_string());
}
