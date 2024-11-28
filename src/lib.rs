use crate::core::container::{
    RCServices,
    RCContainer
};
use crate::core::container::networks::{RCNetwork, RCNetworkDriver};

mod core;
mod tests;

pub fn main() -> Result<(), std::io::Error> {
    let services = RCServices {
        containers: vec![
            RCContainer::new()
                .set_id("120df8c")
                .set_name("first_service")
                .set_image("nginx:latest")
                .add_port(8080, 80)
                .add_environment("KEY1", "value1")
                .add_environment("KEY2", "value2")
                .add_volume("./nginx.conf:/etc/nginx/nginx.conf")
                .add_volume("myapp:/home/node/app")
                .add_networks(RCNetwork{
                    name: "my_network",
                    driver: Some(RCNetworkDriver::BRIDGE.as_str()),
                    driver_opts: None,
                    ipam: None,
                    external: None,
                }),
            RCContainer::new()
                .set_id("78s6xc4") // move .set_id() at the creation process of the container
                .set_name("second_service")
                .set_image("rust:latest")
                .add_port(8080, 80)
                .add_port(777, 1010)
                .add_environment("KEY1", "value1")
                .add_environment("KEY2", "value2")
        ],
    };

    let compose = services.generate_compose()?;

    for container in &compose.containers {
        container.stop();
    }

    Ok(())
}