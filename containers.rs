use serialize::{json, Decodable};
use serialize::json::DecoderError;

#[deriving(Decodable)]
pub struct PortMapping {
  IP: String,
  PrivatePort: uint,
  PublicPort: uint,
  Type: String
}

#[deriving(Decodable)]
pub struct Container {
  Command: String,
  Created: u64,
  Id: String,
  Image: String,
  Names: Vec<String>,
  Ports: Vec<PortMapping>,
  Status: String
}

#[deriving(Decodable)]
pub type Containers = Vec<Container>;

pub fn parse(json_string: &str) -> Result<Containers, DecoderError> {
  let json_object = json::from_str(json_string);
  let mut decoder = json::Decoder::new(json_object.unwrap());
  Decodable::decode(&mut decoder)
}

#[test]
fn test_port_mapping_deserialization() {
  let json_string = "{\"IP\": \"0.0.0.0\", \"PrivatePort\": 9000, \"PublicPort\": 9001, \"Type\": \"tcp\"}";
  let json_object = json::from_str(json_string);
  let mut decoder = json::Decoder::new(json_object.unwrap());
  let port_mapping: PortMapping = Decodable::decode(&mut decoder).unwrap();
  assert!(port_mapping.IP == "0.0.0.0".to_string());
  assert!(port_mapping.PrivatePort == 9000);
  assert!(port_mapping.PublicPort == 9001);
  assert!(port_mapping.Type == "tcp".to_string());
}

#[test]
fn test_container_deserialization() {
  let json_string = "{\"Command\":\"/bin/bash\",\"Created\":1402812645,\"Id\":\"8397b5a5a497b701d3514ca18ba11dc24b32378a7328ef28510c0f49ef30cddf\",\"Image\":\"ubuntu:14.04\",\"Names\":[\"/silly_kirch\"],\"Ports\":[],\"Status\": \"Up 26 seconds\",\"Ports\":[{\"IP\": \"0.0.0.0\", \"PrivatePort\": 9000, \"PublicPort\": 9001, \"Type\": \"tcp\"}]}";
  let json_object = json::from_str(json_string);
  let mut decoder = json::Decoder::new(json_object.unwrap());

  let c: Container = Decodable::decode(&mut decoder).unwrap();
  assert!(c.Command == "/bin/bash".to_string());
  assert!(c.Created == 1402812645u64);
  assert!(c.Id == "8397b5a5a497b701d3514ca18ba11dc24b32378a7328ef28510c0f49ef30cddf".to_string());
  assert!(c.Image == "ubuntu:14.04".to_string());
  assert!(c.Names.len() == 1);
  assert!(*c.Names.get(0) == "/silly_kirch".to_string());
  assert!(c.Status == "Up 26 seconds".to_string());
  assert!(c.Ports.len() == 1);

  let port_mapping = c.Ports.get(0);
  assert!(port_mapping.IP == "0.0.0.0".to_string());
  assert!(port_mapping.PrivatePort == 9000);
  assert!(port_mapping.PublicPort == 9001);
  assert!(port_mapping.Type == "tcp".to_string());
}

#[test]
fn test_parse_and_containers_deserialization() {
  let json_string = "[{\"Command\":\"/bin/bash\",\"Created\":1402812645,\"Id\":\"8397b5a5a497b701d3514ca18ba11dc24b32378a7328ef28510c0f49ef30cddf\",\"Image\":\"ubuntu:14.04\",\"Names\":[\"/silly_kirch\"],\"Ports\":[],\"Status\": \"Up 26 seconds\",\"Ports\":[{\"IP\": \"0.0.0.0\", \"PrivatePort\": 9000, \"PublicPort\": 9001, \"Type\": \"tcp\"}]}]";
  let containers: Containers = parse(json_string).unwrap();
  assert!(containers.len() == 1);
 
  let c = containers.get(0);
  assert!(c.Command == "/bin/bash".to_string());
  assert!(c.Created == 1402812645u64);
  assert!(c.Id == "8397b5a5a497b701d3514ca18ba11dc24b32378a7328ef28510c0f49ef30cddf".to_string());
  assert!(c.Image == "ubuntu:14.04".to_string());
  assert!(c.Names.len() == 1);
  assert!(*c.Names.get(0) == "/silly_kirch".to_string());
  assert!(c.Status == "Up 26 seconds".to_string());
  assert!(c.Ports.len() == 1);
  
  let port_mapping = c.Ports.get(0);
  assert!(port_mapping.IP == "0.0.0.0".to_string());
  assert!(port_mapping.PrivatePort == 9000);
  assert!(port_mapping.PublicPort == 9001);
  assert!(port_mapping.Type == "tcp".to_string());
}