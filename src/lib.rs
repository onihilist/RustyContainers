use crate::core::container::{
    RCServices,
    RCContainer
};
use crate::colorama::Colored;
use crate::core::container::networks::{RCNetwork, RCNetworkDriver};

#[macro_use]
extern crate colorama;
extern crate chrono;

mod core;
mod tests;
mod utils;

pub fn main() -> Result<(), std::io::Error> {
    rc_debug_mode!(true);
    let services = RCServices {
        containers: vec![
            RCContainer::new()
                .set_id("120df8c")
                .set_name("nginx_service")
                .set_image("nginx:latest")
                .add_port(8080, 80)
                .add_port(443, 443)
                .add_networks(RCNetwork{
                    name: "my_network",
                    driver: Some(RCNetworkDriver::BRIDGE.as_str()),
                    driver_opts: None,
                    ipam: None,
                    external: None,
                }),
            RCContainer::new()
                .set_id("78s6xc4") // move .set_id() at the creation process of the container
                .set_name("mysql_service")
                .set_image("mysql:latest")
                .add_port(3306, 3306)
                .add_environment("MYSQL_ROOT_PASSWORD", "rootpass")
                .add_environment("MYSQL_RANDOM_ROOT_PASSWORD", "rootpass")
        ],
    }.build()?;

    Ok(())
}