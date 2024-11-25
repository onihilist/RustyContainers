
#[derive(Debug, Clone)]
pub struct RCNetwork {
    pub name: String,
    pub driver: Option<String>,
    pub driver_opts: Option<std::collections::HashMap<String, String>>,
    pub ipam: Option<RCIpamConfig>,
    pub external: Option<bool>,
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

pub fn build_network() {
    let my_network = RCNetwork {
        name: String::from("my_network"),
        driver: Some(String::from("bridge")),
        driver_opts: None,
        ipam: Some(RCIpamConfig {
            driver: Some(String::from("default")),
            config: vec![
                RCIpamConfigEntry {
                    subnet: Some(String::from("192.168.1.0/24")),
                    ip_range: Some(String::from("192.168.1.100/24")),
                    gateway: Some(String::from("192.168.1.1")),
                },
            ],
        }),
        external: Some(false),
    };

    println!("{:?}", my_network);
}
