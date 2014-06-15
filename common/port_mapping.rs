#![allow(uppercase_variables)]

#[cfg(test)]
use serialize::{json, Decodable};

#[deriving(Decodable)]
pub struct PortMapping {
  pub IP: String,
  pub PrivatePort: uint,
  pub PublicPort: uint,
  pub Type: String
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
