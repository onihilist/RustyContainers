use crate::core::container::{
    RCServices,
    RCContainer
};

mod core;
mod tests;

pub fn main() -> Result<(), std::io::Error> {
    RCServices {
        containers: vec![
            RCContainer::new()
                .set_name("first_service")
                .set_image("nginx:latest")
                .add_port("8080", "80"),
            RCContainer::new()
                .set_name("second_service")
                .set_image("rust:latest")
                .add_port("8080", "80")
                .add_port("777", "1010")
        ],
    }.generate_compose()?;

    Ok(())
}
