use crate::core::container::RCContainer;

#[derive(Debug, Clone)]
pub struct RCNetwork {
    pub name: &'static str,
    pub driver: Option<&'static str>,
    pub driver_opts: Option<std::collections::HashMap<String, String>>,
    pub ipam: Option<RCIpamConfig>,
    pub external: Option<bool>,
}

#[derive(Debug, Clone)]
pub enum RCNetworkDriver {
    BRIDGE,
    HOST,
    OVERLAY,
    MACVLAN,
    NONE,
}

impl RCNetworkDriver {
    pub fn as_str(&self) -> &str {
        match self {
            RCNetworkDriver::BRIDGE => "bridge",
            RCNetworkDriver::HOST => "host",
            RCNetworkDriver::OVERLAY => "overlay",
            RCNetworkDriver::MACVLAN => "macvlan",
            RCNetworkDriver::NONE => "none",
        }
    }
}

#[derive(Debug, Clone)]
pub struct RCIpamConfig {
    pub driver: Option<String>,
    pub config: Vec<RCIpamConfigEntry>,
}

#[derive(Debug, Clone)]
pub struct RCIpamConfigEntry {
    pub subnet: Option<String>,
    pub ip_range: Option<String>,
    pub gateway: Option<String>,
}

impl RCContainer {
    pub(crate) fn add_networks(mut self, network_given: RCNetwork) -> Self {
        if let Some(ref mut networks) = self.networks {
            networks.push(network_given);
        }
        self
    }
}
