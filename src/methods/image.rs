use serialize::{json, Decodable};
use serialize::json::DecoderError;

use super::super::common::images::Images as Images;
use super::super::http as http;

///
/// GET /images/json
///

fn parse_get_images(json_string: &str) -> Result<Images, DecoderError> {
  let json_object = json::from_str(json_string);
  let mut decoder = json::Decoder::new(json_object.unwrap());
  Decodable::decode(&mut decoder)
}

pub fn get_images(socket_path: &str) -> Images {
  let method = http::GET;
  let path = "/images/json?all=0";

  let response = http::make_request(socket_path.as_slice(), method, path);
  if response.status_code >= 200 && response.status_code < 300 {
    let result = parse_get_images(response.body.as_slice());
    match result {
      Err(_) => fail!("JSON response could not be decoded"),
      Ok(containers) => containers
    }  
  }
  else {
    fail!("HTTP response code was {}", response.status_code);
  }
}