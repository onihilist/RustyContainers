
mod networks;

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use networks::RCNetwork;

#[derive(Debug)]
pub struct RCServices {
    pub containers: Vec<RCContainer>,
}

#[derive(Debug)]
pub struct RCContainer {
    id: Option<String>,
    name: String,
    image: String,
    build: Option<String>,
    command: Option<String>,
    depends_on: Option<String>,
    environment: Option<HashMap<String, String>>,
    volumes: Option<Vec<String>>,
    ports: Vec<RCPortMapping>,
    networks: Option<RCNetwork>,
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

#[derive(Debug)]
struct RCPortMapping {
    host: String,
    container: String,
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
            environment: None,
            volumes: None,
            ports: vec![],
            networks: None,
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

    pub(crate) fn set_name(mut self, service_name: &str) -> Self {
        self.name = service_name.to_string();
        self
    }

    pub(crate) fn set_image(mut self, service_image: &str) -> Self {
        self.image = service_image.to_string();
        self
    }

    pub(crate) fn add_port(mut self, host_port: &str, container_port: &str) -> Self {
        self.ports.push(RCPortMapping {
            host: host_port.to_string(),
            container: container_port.to_string(),
        });
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
    pub fn generate_compose(&self) -> Result<(), std::io::Error> { // Take an immutable reference
        let file = fs::File::create("docker-compose.yml")?;
        let mut writer = std::io::BufWriter::new(file);

        writer.write_all(b"version: '3.8'\n")?;
        writer.write_all(b"services:\n")?;

        for container in &self.containers {
            writer.write_all(format!("\t{}:\n", container.name).as_bytes())?;
            writer.write_all(format!("\t\timage: {}\n", container.image).as_bytes())?;
            writer.write_all(b"\t\tports:\n")?;
            for port in &container.ports {
                writer.write_all(format!("\t\t\t- '{}:{}'\n", port.host, port.container).as_bytes())?;
            }
            writer.write_all(b"\n")?;
        }

        Ok(())
    }
}