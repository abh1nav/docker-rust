#Docker-Rust (WIP)
  
A client library for the Docker Remote API written in Rust.  
Currently targetting API v1.12  
  
## Build
  
```bash
git clone git@github.com:abh1nav/docker-rust.git
cd docker-rust
make
```
  
This will produce a libdocker-*-.rlib that you can use in your project.
  
## Features 
  
### Get a list of running containers
  
```rust
extern crate docker;

use docker::Docker;

let docker: Docker = Docker {
	socket_path: "/var/run/docker.sock"
};

let containers = docker.get_containers();

println!("Running container count: {}", containers.len());
```
  