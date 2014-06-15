use containers = super::common::containers;
use Containers = super::common::containers::Containers;
use http = super::http;

pub struct Docker {
  socket_path: &'static str
}

impl Docker {

  pub fn get_containers(&self) -> Containers {
    let method = http::GET;
    let path = "/containers/json?all=1&size=1";

    let response = http::make_request(self.socket_path, method, path);
    let result = containers::parse(response.body.as_slice());
    match result {
      Err(_) => fail!("JSON response could not be decoded"),
      Ok(containers) => containers
    }
  }

}