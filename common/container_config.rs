#![allow(uppercase_variables)]

#[cfg(test)]
use serialize::{json, Decodable};
#[cfg(test)]
use serialize::json::{DecoderError};

//
// Representation of a containers "config" as returned by the container 
// inspect call - this is currently broken because we can't use 
// serialize::json::Decodable to decode something where the keys are 
// dynamic.
//
// Specifically, the ExposedPorts key in the config object is represented as:
//
//  "ExposedPorts": {
//    "9000/tcp": {}
//  }
//
//  and there's no way to parse this currently using stdlib's JSON decoder.
//

#[deriving(Decodable)]
pub struct Config {
  AttachStderr: bool,
  AttachStdin: bool,
  AttachStdout: bool,
  Cmd: Vec<String>,
  CpuShares: u8,
  Cpuset: String,
  Domainname: String,
  Entrypoint: Vec<String>,
  Env: Vec<String>,
  // ExposedPorts: ???
  Hostname: String,
  Image: String,
  Memory: u64,
  MemorySwap: u64,
  NetworkDisabled: bool,
  OnBuild: String,
  OpenStdin: bool,
  PortSpecs: bool,
  StdinOnce: bool,
  Tty: bool,
  User: String,
  Volumes: String,
  WorkingDir: String
}

//
// Test(s)
//

#[test]
fn test_config_deserialization() {
  let json_string = "\"Config\":{\"AttachStderr\":false,\"AttachStdin\":false,\"AttachStdout\":false,\"Cmd\":[\"-e\",\"/docker.sock\"],\"CpuShares\":0,\"Cpuset\":\"\",\"Domainname\":\"\",\"Entrypoint\":[\"./dockerui\"],\"Env\":[\"HOME=/\",\"PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/local/go/bin\",\"GOPATH=/go\",\"GOROOT=/usr/local/go\"],\"ExposedPorts\":{\"9000/tcp\":{}},\"Hostname\":\"5fc6a1226f01\",\"Image\":\"crosbymichael/dockerui\",\"Memory\":0,\"MemorySwap\":0,\"NetworkDisabled\":false,\"OnBuild\":null,\"OpenStdin\":false,\"PortSpecs\":null,\"StdinOnce\":false,\"Tty\":false,\"User\":\"\",\"Volumes\":null,\"WorkingDir\":\"/app\"}";
  let json_object = json::from_str(json_string);
  let mut decoder = json::Decoder::new(json_object.unwrap());
  let c: Config = Decodable::decode(&mut decoder).unwrap();
}