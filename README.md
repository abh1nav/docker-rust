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

let client: Docker = Docker {
	socket_path: "/var/run/docker.sock"
};
  
let containers = client.get_containers();
  
println!("Running container count: {}", containers.len());
```
  
### Stop and remove a container
  
```rust
let container_id = "5fc6a1226f01";

// Stop the container
client.stop_container(container_id);

// Remove the container and its volumes
client.remove_container(container_id);
```
