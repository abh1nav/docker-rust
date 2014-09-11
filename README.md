#Docker-Rust (WIP)
  
A client library for the Docker Remote API written in Rust.  
Currently targetting API v1.12+
  
A lot of functionality is currently on hold until Rust's HTTP and JSON libs are up to snuff.
  
### Run Tests

```bash
make test
```

## Use

Declare cargo dependency in your `Cargo.toml`

```
[dependencies.docker]

git = "https://github.com/abh1nav/docker-rust.git"
```

## Features 
  
### Get a list of running containers
  
```rust
extern crate docker;

use docker::Docker;

// Currently only works if docker is run using a unix socket (the default)
let client: Docker = Docker {
	socket_path: "/var/run/docker.sock"
};
  
// common::containers::Containers is a Vec<common::containers::Container>
let containers: Containers = client.get_containers();
  
println!("Running container count: {}", containers.len());
```
  
### Restart, stop and remove a container
  
```rust
let container_id = "5fc6a1226f01".to_string();

// Restart container
client.restart_container(container_id.as_slice());
// OR wait 3 seconds for the container to stop before forcing restart
client.restart_container_with_timeout(container_id.as_slice(), 3);

// Stop the container
client.stop_container(container_id.as_slice());
// OR wait 3 seconds for the container to stop before killing
client.stop_container_with_timeout(container_id.as_slice(), 3);

// Remove the container and its volumes
client.remove_container(container_id.as_slice());
// OR remove with the force flag
client.remove_container_with_force(container_id.as_slice());
```

### Utility endpoints

```rust

// Get system info -> common::sys_info::SysInfo
let sys_info = client.get_sys_info();
println!("Number of containers: {}\nNumber of Images: {}", sys_info.Containers, sys_info.Images);

// Get docker version -> common::version::Version
let version = client.get_version();
println!("Docker version: {}", version.Version);

```
