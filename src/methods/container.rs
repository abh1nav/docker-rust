use serialize::{json, Decodable};
use serialize::json::DecoderError;

use super::super::common::containers::Containers as Containers;
use super::super::http as http;


///
/// GET /containers/json
///

fn parse_get_containers(json_string: &str) -> Result<Containers, DecoderError> {
  let json_object = json::from_str(json_string);
  let mut decoder = json::Decoder::new(json_object.unwrap());
  Decodable::decode(&mut decoder)
}

pub fn get_containers(socket_path: &str) -> Containers {
  let method = http::GET;
  let path = "/containers/json?all=1&size=1";

  let response = http::make_request(socket_path.as_slice(), method, path);
  if response.status_code >= 200 && response.status_code < 300 {
    let result = parse_get_containers(response.body.as_slice());
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

pub fn stop_container_impl(socket_path: &str, id: &str, wait_time: Option<uint>) {
  let method = http::POST;
  let mut path = String::new();
  path.push_str("/containers/");
  path.push_str(id);
  path.push_str("/stop");

  match wait_time {
    Some(timeout_value) => {
      // If a wait time was specified, include it in the query string
      path.push_str("?t=");
      path.push_str(timeout_value.to_string().as_slice());
    }
    None => {
      // Don't do anything
    }
  };

  let response = http::make_request(socket_path.as_slice(), method, path.as_slice());
  if response.status_code < 200 || response.status_code >= 400 {
    fail!("HTTP response code was {}\n{}", response.status_code, response.body);
  }
}

///
/// POST /containers/(id)/restart
///

pub fn restart_container_impl(socket_path: &str, id: &str, wait_time: Option<uint>) {
  let method = http::POST;
  let mut path = String::new();
  path.push_str("/containers/");
  path.push_str(id);
  path.push_str("/restart");

  match wait_time {
    Some(timeout_value) => {
      // If a wait time was specified, include it in the query string
      path.push_str("?t=");
      path.push_str(timeout_value.to_string().as_slice());
    }
    None => {
      // Don't do anything
    }
  };

  let response = http::make_request(socket_path.as_slice(), method, path.as_slice());
  if response.status_code < 200 || response.status_code >= 300 {
    fail!("HTTP response code was {}", response.status_code);
  }
}

///
/// DELETE /containers/(id)/
///

pub fn remove_container_impl(socket_path: &str, id: &str, force: bool) {
  let method = http::DELETE;
  let mut path = String::new();
  path.push_str("/containers/");
  path.push_str(id);
  path.push_str("?v=1");

  if force {
    path.push_str("&f=1");
  }

  let response = http::make_request(socket_path.as_slice(), method, path.as_slice());
  if response.status_code < 200 || response.status_code >= 300 {
    fail!("HTTP response code was {}", response.status_code);
  }
}