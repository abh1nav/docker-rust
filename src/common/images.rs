#![allow(non_snake_case)]

#[cfg(test)]
use serialize::{json, Decodable};

///
/// Image representation returned by the /images/json call
///

#[deriving(Decodable)]
pub struct Image {
  pub RepoTags: Vec<String>,
  pub Created: u64,
  pub Id: String,
  pub Size: u64,
  pub VirtualSize: u64
}

///
/// Images is a vector of Image objects - need this for 
/// auto-deserialization
///

#[deriving(Decodable)]
pub type Images = Vec<Image>;

///
/// Tests
///

#[test]
fn test_container_deserialization() {
  let json_string = "{\"Created\":1385109237,\"Id\":\"e02d1bfed8743d7a0176de2e4c03842359f10ba10947cdc71e147bb538a2910c\",\"ParentId\":\"4995dc7f431090c5d32f7a55af3ab1b9b04f96af120d138f436fa991d5e15026\",\"RepoTags\":[\"dustin/couchbase:latest\"],\"Size\":0,\"VirtualSize\":749634786}";
  let json_object = json::from_str(json_string);
  let mut decoder = json::Decoder::new(json_object.unwrap());

  let i: Image = Decodable::decode(&mut decoder).unwrap();
  assert!(i.Created == 1385109237u64);
  assert!(i.Id == "e02d1bfed8743d7a0176de2e4c03842359f10ba10947cdc71e147bb538a2910c".to_string());
  assert!(i.RepoTags.len() == 1);
  assert!(*i.RepoTags.get(0) == "dustin/couchbase:latest".to_string());
  assert!(i.Size == 0u64);
  assert!(i.VirtualSize == 749634786u64);
}

#[test]
fn test_containers_deserialization() {
  let json_string = "[{\"Created\":1385109237,\"Id\":\"e02d1bfed8743d7a0176de2e4c03842359f10ba10947cdc71e147bb538a2910c\",\"ParentId\":\"4995dc7f431090c5d32f7a55af3ab1b9b04f96af120d138f436fa991d5e15026\",\"RepoTags\":[\"dustin/couchbase:latest\"],\"Size\":0,\"VirtualSize\":749634786}]";
  let json_object = json::from_str(json_string);
  let mut decoder = json::Decoder::new(json_object.unwrap());

  let images: Images = Decodable::decode(&mut decoder).unwrap();
  assert!(images.len() == 1);
  let i: Image = &images[0];
  assert!(i.Created == 1385109237u64);
  assert!(i.Id == "e02d1bfed8743d7a0176de2e4c03842359f10ba10947cdc71e147bb538a2910c".to_string());
  assert!(i.RepoTags.len() == 1);
  assert!(*i.RepoTags.get(0) == "dustin/couchbase:latest".to_string());
  assert!(i.Size == 0u64);
  assert!(i.VirtualSize == 749634786u64);
}