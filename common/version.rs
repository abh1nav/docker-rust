#![allow(uppercase_variables)]

#[cfg(test)]
use serialize::{json, Decodable};

#[deriving(Decodable)]
pub struct Version {
	pub Version: String,
	pub GitCommit: String,
	pub GoVersion: String
}

#[test]
fn test_version_deserialization() {
  let json_string = "{\"Version\":\"0.2.2\",\"GitCommit\":\"5a2a5cc+CHANGES\",\"GoVersion\":\"go1.0.3\"}";
  let json_object = json::from_str(json_string);
  let mut decoder = json::Decoder::new(json_object.unwrap());
  let version: Version = Decodable::decode(&mut decoder).unwrap();

  assert!(version.Version == "0.2.2".to_string());
  assert!(version.GitCommit == "5a2a5cc+CHANGES".to_string());
  assert!(version.GoVersion == "go1.0.3".to_string());
}