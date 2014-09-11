#![comment = "Rust Docker Client"]

#![license = "MIT/ASL2"]

#![crate_type = "lib"]

#![feature (globs, macro_rules)]

extern crate collections;
extern crate debug;
extern crate serialize;

pub use docker::Docker;

mod http;

pub mod common;
pub mod docker;

