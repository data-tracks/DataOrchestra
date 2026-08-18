use std::thread;

use crate::core::{object::Object, traits::Spawner, types::Node};

/// The config object. Contains all object types tasks
#[derive(Debug)]
pub struct Config {
    pub objects: Vec<Object>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            objects: Vec::new(),
        }
    }
}

impl Config {
    /// Get mutable reference from all nodes from all object types
    pub fn get_object_nodes_mut(&mut self) -> Vec<&mut Node> {
        let mut nodes = Vec::new();

        for object in self.objects.iter_mut() {
            if let Some(node) = object.node.as_mut() {
                nodes.push(node);
            }
        }

        nodes
    }

    /// Get reference from all unique nodes from all object types
    pub fn get_nodes(&self) -> Vec<&Node> {
        let mut nodes = Vec::new();
        let mut hosts = Vec::new();

        for object in self.objects.iter() {
            if let Some(node) = object.node.as_ref() {
                if !hosts.contains(&node.host) {
                    nodes.push(node);
                    hosts.push(node.host);
                }
            }
        }

        nodes
    }

    /// Get mutable reference to all objects in config which implement the [`Spawner`] trait.
    pub fn get_mut_spawners<'a>(
        &'a mut self,
    ) -> impl Iterator<Item = (&'a mut (dyn Spawner + Send + 'a), String)> {
        let mut vec_objects: Vec<&'a mut (dyn Spawner + Send + 'a)> = Vec::new();
        let mut vec_names: Vec<String> = Vec::new();

        for object in self.objects.iter_mut() {
            let name = object.name.clone();
            let spawner = object as _;
            vec_objects.push(spawner);
            vec_names.push(name);
        }

        vec_objects.into_iter().zip(vec_names)
    }
}

impl Spawner for Config {
    fn build(&mut self) {
        let spawners = self.get_mut_spawners();
        thread::scope(|s| {
            for (spawner, name) in spawners {
                thread::Builder::new()
                    .name(name)
                    .spawn_scoped(s, move || {
                        spawner.build();
                    })
                    .unwrap();
            }
        });
    }

    fn setup(&mut self) {
        let spawners = self.get_mut_spawners();
        thread::scope(|s| {
            for (spawner, name) in spawners {
                thread::Builder::new()
                    .name(name)
                    .spawn_scoped(s, move || {
                        spawner.setup();
                    })
                    .unwrap();
            }
        });
    }

    fn deploy(&mut self) {
        let spawners = self.get_mut_spawners();
        thread::scope(|s| {
            for (spawner, name) in spawners {
                thread::Builder::new()
                    .name(name)
                    .spawn_scoped(s, move || {
                        spawner.deploy();
                    })
                    .unwrap();
            }
        });
    }
}
