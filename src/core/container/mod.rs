
pub mod networks;
mod port;

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use networks::RCNetwork;
use port::RCPortMapping;

#[derive(Debug)]
pub struct RCServices {
    pub containers: Vec<RCContainer>,
}

#[derive(Debug)]
pub struct RCContainer {
    pub(crate) id: Option<String>,
    pub(crate) name: String,
    image: String,
    build: Option<String>,
    command: Option<String>,
    depends_on: Option<String>,
    environment: Option<HashMap<String, String>>,
    volumes: Option<Vec<String>>,
    ports: Vec<RCPortMapping>,
    networks: Option<Vec<RCNetwork>>,
    restart_policy: Option<String>,
    healthcheck: Option<String>,
    deploy_policy: Option<String>,
    logging: Option<String>,
    labels: Option<HashMap<String, String>>,
    secrets: Option<HashMap<String, String>>,
    working_directory: Option<String>,
    network_mode: Option<String>,
    entrypoint: Option<String>,
}

impl RCContainer {

    pub(crate) fn new() -> Self {
        RCContainer {
            id: None,
            name: "".to_string(),
            image: "".to_string(),
            build: None,
            command: None,
            depends_on: None,
            environment: Some(HashMap::new()),
            volumes: Some(vec![]),
            ports: vec![],
            networks: Some(vec![]),
            restart_policy: None,
            healthcheck: None,
            deploy_policy: None,
            logging: None,
            labels: None,
            secrets: None,
            working_directory: None,
            network_mode: None,
            entrypoint: None,
        }
    }

    pub(crate) fn set_id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub(crate) fn set_name(mut self, service_name: &str) -> Self {
        self.name = service_name.to_string();
        self
    }

    pub(crate) fn set_image(mut self, service_image: &str) -> Self {
        self.image = service_image.to_string();
        self
    }

    pub(crate) fn add_environment(mut self, key: &str, value: &str) -> Self {
        if let Some(ref mut env) = self.environment {
            env.insert(key.to_string(), value.to_string());
        }
        self
    }

    pub(crate) fn add_volume(mut self, volume: &str) -> Self {
        if let Some(ref mut volumes) = self.volumes {
            volumes.push(volume.to_string());
        }
        self
    }

    pub(crate) fn set_restart_policy(mut self, policy: &str) -> Self {
        self.restart_policy = Some(policy.to_string());
        self
    }

}

impl RCServices {
    pub fn generate_compose(self) -> Result<Self, std::io::Error> {
        let file = fs::File::create("docker-compose.yml")?;
        let mut writer = std::io::BufWriter::new(file);

        writer.write_all(b"version: '3.8'\n")?;
        writer.write_all(b"services:\n")?;

        for container in &self.containers {
            writer.write_all(format!("  {}:\n", container.name).as_bytes())?;
            writer.write_all(format!("    image: {}\n", container.image).as_bytes())?;

            if !container.ports.is_empty() {
                writer.write_all(b"    ports:\n")?;
                for port in &container.ports {
                    writer.write_all(format!("      - '{}:{}'\n", port.host, port.container).as_bytes())?;
                }
            }

            // Write environment variables if they exist
            if let Some(ref env) = container.environment {
                writer.write_all(b"    environment:\n")?;
                for (key, value) in env {
                    writer.write_all(format!("      {}: {}\n", key, value).as_bytes())?;
                }
            }

            if let Some(ref vol) = container.volumes {
                if !vol.is_empty() {
                    writer.write_all(b"    volumes:\n")?;
                    for volume in vol {
                        writer.write_all(format!("      - {}\n", volume).as_bytes())?;
                    }
                }
            }
            writer.write_all(b"\n")?;
        }

        for container in &self.containers {
            if let Some(ref net) = container.networks {
                if !net.is_empty() {
                    writer.write_all(b"networks:\n")?;
                    for network in net {
                        writer.write_all(format!("  {}:\n", network.name).as_bytes())?;
                        writer.write_all(format!("    driver: {}", network.driver.unwrap()).as_bytes())?;
                    }
                }
            }
        }

        Ok(self)
    }
}
