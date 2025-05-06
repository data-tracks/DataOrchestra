use crate::core::data::Data;
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

    /// Combine containers managed by manager with [`Data`] to [`Iterator`] 
    pub fn iter_combine_data<'a>(&'a self, data: &'a Vec<Data>) -> impl Iterator<Item = (&'a Container, &'a Data)> {
        let mut vec_container = Vec::<&Container>::new();
        let mut vec_data = Vec::<&Data>::new();

        for d in data {
            if let Some(ref mut container) = self.containers.get(&d.name) {
                vec_data.push(d);
                vec_container.push(container);
            }
        };

        vec_container.into_iter().zip(vec_data)
    }
}
