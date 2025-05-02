use super::command::command_func::{output_command, spawn_command};

pub mod traits;
pub use traits::Run;

pub mod source;
pub use source::DockerSource;

mod compose;
pub use compose::ComposeGroup;
pub use compose::ComposeGroupBuilder;

pub mod container;
pub use container::Container;

pub mod portmapping;
pub use portmapping::PortMapping;

pub mod manager;
pub use manager::DockerManager;

pub mod config;
pub use config::ContainerConfig;
pub use config::ContainerConfigBuilder;

/// Create a new docker network.
pub fn create_network<T: Into<String>>(network: T) -> Result<(), String>{
    let network = network.into();
    let networks: String = output_command("docker network ls");
    if !networks.contains(&network) {
        let create_bridge = spawn_command(&format!("docker network create -d bridge {}", &network))
            .wait();
    
        if create_bridge.is_err() || (create_bridge.is_ok() && !&create_bridge.as_ref().unwrap().success()) {
            return Err(String::from("Unable to create bridge"));
        }
    }

    Ok(())
}

pub fn get_networks() -> Vec<String> {
   let mut networks = Vec::<String>::new();
   let output = output_command("docker network ls");
   networks
}

/// Deletes all docker containers
pub fn remove_all() {
    let containers = output_command("docker container ls -a -q");
    for container in containers.split("\n") {
        let _ = output_command(format!("docker rm {}", container));
    }
}

/// Stops all docker containers
pub fn stop_all() {
    let containers = output_command("docker container ls -a -q");
    for container in containers.split("\n") {
        let _ = output_command(format!("docker stop {}", container));
    }
}
