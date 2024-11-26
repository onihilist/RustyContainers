use crate::core::container::RCContainer;

#[derive(Debug)]
pub struct RCPortMapping {
    pub(crate) host: String,
    pub(crate) container: String,
}

impl RCContainer {
    pub(crate) fn add_port(mut self, host_port: u16, container_port: u16) -> Self {
        self.ports.push(RCPortMapping {
            host: host_port.to_string(),
            container: container_port.to_string(),
        });
        self
    }
}
