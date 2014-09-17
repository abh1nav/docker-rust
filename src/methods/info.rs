use serialize::{json, Decodable};
use serialize::json::DecoderError;

use super::super::common::version::Version as Version;
use super::super::common::sys_info::SysInfo as SysInfo;

use super::super::http as http;

///
/// GET /info
///

fn parse_get_sys_info(json_string: &str) -> Result<SysInfo, DecoderError> {
  let json_object = json::from_str(json_string);
  let mut decoder = json::Decoder::new(json_object.unwrap());
  Decodable::decode(&mut decoder)
}

pub fn get_sys_info(socket_path: &str) -> SysInfo {
  let method = http::GET;
  let path = "/info";
  let response = http::make_request(socket_path.as_slice(), method, path);
  if response.status_code == 200 {
    let result = parse_get_sys_info(response.body.as_slice());
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

pub fn get_version(socket_path: &str) -> Version {
  let method = http::GET;
  let path = "/version";
  let response = http::make_request(socket_path.as_slice(), method, path);
  if response.status_code == 200 {
    let result = parse_get_version(response.body.as_slice());
    match result {
      Err(_) => fail!("JSON response could not be decoded"),
      Ok(version) => version
    }  
  }
  else {
    fail!("HTTP response code was {}", response.status_code);
  }

}