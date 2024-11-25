# RustyContainers

### Welcome to RustyContainer !

( 🚨⚠️ **WARNING** : This lib is <ins>currently in development</ins>, **no warranty will be provided** )
- Not still available in crates.io

First of all, you probably wondering what is this library ?

RustyContainer is a lib for create custom container in Rust, running your entire project in a container, and manage all about your containers/services. 
It's also made for **high-performance, scalability/flexibility, portability.**

You can run your Rust project into a container but also run multiples other containers, there is an exemple : 

```rust
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
```

This main function generate the following 'docker-compose.yml' file :

```docker-compose.yml
version: '3.8'
services:
  first_service:
    image: nginx:latest
    ports:
      - '8080:80'

  second_service:
    image: rust:latest
    ports:
      - '8080:80'
      - '777:1010'
```

### Features (docker-compose) :
#### Options available for the moment :

OK : option is usable
DEV : option is currently in development
TODO : option in the list of features but not done

| Available | Option                                             | Status | Description                           | Incoming modification |
| --------- | -------------------------------------------------- | ------ | ------------------------------------- | --------------------- |
| ✔️        | `.set_name(service_name: &str)`                    | OK     | Set the service name.                 | ❌                     |
| ✔️        | `.set_image(service_image: &str)`                  | OK     | Set the image to run for the service. | ❌                     |
| ✔️        | `.add_port(host_port: &str, container_port: &str)` | OK     | Set ports for the service.            | ❌                     |
| ✔️         | `.add_environment(key: &str, value: &str)`         | OK    | Add a env var to the service.         | ❌                    |
| ✔️         | `.add_volume(volume: &str)`                        | OK    | Add a volume to the service.          | ❌                    |
| ✔️         | `.add_network(network_given: RCNetwork)`           | OK    | Add a volume to the service.          | ❌                    |
| ❌         | `.set_restart_policy(policy: &str)`                | TODO  |                                       | ❌                     |

