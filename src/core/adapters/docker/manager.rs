use std::collections::HashMap;

use super::Container;

#[derive(Debug)]
pub struct DockerManager {
    pub containers: HashMap<String ,Container>,
}

impl DockerManager {
    pub fn new() -> DockerManager {
        DockerManager { containers: HashMap::new() }
    }

    /// Add a container to the manager
    pub fn add_container<T: Into<String>>(&mut self, key: T, value: Container) -> &mut Self {
        self.containers.insert(key.into(), value);
        self
    }

    /// Get Containers of manager as a reference vector
    pub fn as_vec(&self) -> Vec<&Container> {
        let mut vec = Vec::new();
        for (_, container) in &self.containers {
            vec.push(container);
        } 

        vec
    }
}


