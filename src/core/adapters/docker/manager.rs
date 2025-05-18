use std::collections::HashMap;

use super::{Container, ContainerType};

#[derive(Debug)]
pub struct DockerManager {
    pub containers: HashMap<String , ContainerType>
}

impl DockerManager {
    pub fn new() -> DockerManager {
        DockerManager { containers: HashMap::new() }
    }

    /// Get amount of containers managed by manager
    pub fn amount(&self) -> usize {
        self.containers.len() 
    }

    /// Get the only or first container. Should mainly be used for when the manager only manages a
    /// single container.
    pub fn get_container(&self) -> Option<&Container> {
        let vec = self.as_vec();
        if vec.len() >= 1 {
            return Some(vec[0]);
        }

        None
    }

    /// Add a container to the manager
    pub fn add<S: Into<String>>(&mut self, key: S, value: ContainerType) -> &mut Self {
        self.containers.insert(key.into(), value);
        self
    }

    /// Get Containers of manager as a reference vector
    pub fn as_vec(&self) -> Vec<&Container> {
        let mut vec = Vec::new();
        for (_, container_type) in &self.containers {
            match container_type {
                ContainerType::Container(container) => vec.push(container),
                ContainerType::Compose(compose) => {
                    for container in compose.containers.iter() {
                        vec.push(container);
                    }              
                }
            }
        } 

        vec
    }
}
