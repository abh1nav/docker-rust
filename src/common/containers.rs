#![allow(non_snake_case)]

#[cfg(test)]
use serialize::{json, Decodable};

use super::port_mapping::PortMapping;

///
/// Container representation returned by the /containers/json call
///

#[deriving(Decodable)]
pub struct Container {
  pub Command: String,
  pub Created: u64,
  pub Id: String,
  pub Image: String,
  pub Names: Vec<String>,
  pub Ports: Vec<PortMapping>,
  pub SizeRootFs: u64,
  pub SizeRw: u64,  
  pub Status: String
}

///
/// Containers is a vector of Container objects - need this for 
/// auto-deserilization
///

#[deriving(Decodable)]
pub type Containers = Vec<Container>;

///
/// Tests
///

#[test]
fn test_container_deserialization() {
  let json_string = "{\"Command\":\"/bin/bash\",\"Created\":1402812645,\"Id\":\"8397b5a5a497b701d3514ca18ba11dc24b32378a7328ef28510c0f49ef30cddf\",\"Image\":\"ubuntu:14.04\",\"Names\":[\"/silly_kirch\"],\"Ports\":[],\"Status\": \"Up 26 seconds\",\"Ports\":[{\"IP\": \"0.0.0.0\", \"PrivatePort\": 9000, \"PublicPort\": 9001, \"Type\": \"tcp\"}],\"SizeRootFs\":100,\"SizeRw\":12288}";
  let json_object = json::from_str(json_string);
  let mut decoder = json::Decoder::new(json_object.unwrap());

  let c: Container = Decodable::decode(&mut decoder).unwrap();
  assert!(c.Command == "/bin/bash".to_string());
  assert!(c.Created == 1402812645u64);
  assert!(c.Id == "8397b5a5a497b701d3514ca18ba11dc24b32378a7328ef28510c0f49ef30cddf".to_string());
  assert!(c.Image == "ubuntu:14.04".to_string());
  assert!(c.Names.len() == 1);
  assert!(*c.Names.get(0) == "/silly_kirch".to_string());
  assert!(c.SizeRootFs == 100u64);
  assert!(c.SizeRw == 12288u64);
  assert!(c.Status == "Up 26 seconds".to_string());
  assert!(c.Ports.len() == 1);

  let port_mapping = &c.Ports[0];
  assert!(port_mapping.IP == "0.0.0.0".to_string());
  assert!(port_mapping.PrivatePort == 9000);
  assert!(port_mapping.PublicPort == 9001);
  assert!(port_mapping.Type == "tcp".to_string());
}

#[test]
fn test_containers_deserialization() {
  let json_string = "[{\"Command\":\"/bin/bash\",\"Created\":1402812645,\"Id\":\"8397b5a5a497b701d3514ca18ba11dc24b32378a7328ef28510c0f49ef30cddf\",\"Image\":\"ubuntu:14.04\",\"Names\":[\"/silly_kirch\"],\"Ports\":[],\"Status\": \"Up 26 seconds\",\"Ports\":[{\"IP\": \"0.0.0.0\", \"PrivatePort\": 9000, \"PublicPort\": 9001, \"Type\": \"tcp\"}],\"SizeRootFs\":100,\"SizeRw\":12288}]";
  let json_object = json::from_str(json_string);
  let mut decoder = json::Decoder::new(json_object.unwrap());
  let containers: Containers = Decodable::decode(&mut decoder).unwrap();
  assert!(containers.len() == 1);
 
  let c = &containers[0];
  assert!(c.Command == "/bin/bash".to_string());
  assert!(c.Created == 1402812645u64);
  assert!(c.Id == "8397b5a5a497b701d3514ca18ba11dc24b32378a7328ef28510c0f49ef30cddf".to_string());
  assert!(c.Image == "ubuntu:14.04".to_string());
  assert!(c.Names.len() == 1);
  assert!(*c.Names.get(0) == "/silly_kirch".to_string());
  assert!(c.SizeRootFs == 100u64);
  assert!(c.SizeRw == 12288u64);
  assert!(c.Status == "Up 26 seconds".to_string());
  assert!(c.Ports.len() == 1);
  
  let port_mapping = &c.Ports[0];
  assert!(port_mapping.IP == "0.0.0.0".to_string());
  assert!(port_mapping.PrivatePort == 9000);
  assert!(port_mapping.PublicPort == 9001);
  assert!(port_mapping.Type == "tcp".to_string());
}
