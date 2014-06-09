mod port_mapping;
type PortMapping = port_mapping::PortMapping;

pub struct Container {
  Id: String,
  Image: String,
  Command: String,
  Created: u32,
  Status: String,
  Ports: Vec<PortMapping>,
  SizeRw: u32,
  SizeRootFs: u32
}