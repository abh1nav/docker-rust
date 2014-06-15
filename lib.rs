#![crate_id = "docker#0.1"]

#![comment = "Rust Docker Client"]

#![license = "MIT/ASL2"]

#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![feature (globs, macro_rules)]

extern crate collections;
extern crate debug;
extern crate serialize;

mod http;

pub mod common;
pub mod docker;