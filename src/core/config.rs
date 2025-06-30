use std::thread;

use crate::shared::Spawner;

use super::{generate::Generate, object::Object, process::Process, store::Store, types::Node};

/// The config object. Contains all object types tasks
#[derive(Debug)]
pub struct Config {
    pub api_port: u16,
    pub store: Vec<Store>,
    pub process: Vec<Process>,
    pub generate: Vec<Generate>,
    pub object: Vec<Object>,
}

impl Default for Config {
    fn default() -> Self {
        Config { api_port: 5000, store: Vec::new(), process: Vec::new(), generate: Vec::new(), object: Vec::new() }
    }
}

impl Config {
    /// Get mutable reference from all nodes from all object types 
    pub fn get_object_nodes_mut(&mut self) -> Vec<&mut Node> {
        let mut nodes = Vec::new();

        for generate in self.generate.iter_mut() {
            if let Some(node) = generate.object.node.as_mut() {
                nodes.push(node);
            }
        }

        for process in self.process.iter_mut() {
            if let Some(node) = process.object.node.as_mut() {
                nodes.push(node);
            }
        }

        for store in self.store.iter_mut() {
            if let Some(node) = store.object.node.as_mut() {
                nodes.push(node);
            }
        }
 
        for object in self.object.iter_mut() {
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

        for generate in self.generate.iter() {
            if let Some(node) = generate.object.node.as_ref() {
                if !hosts.contains(&node.host) {
                    nodes.push(node);
                    hosts.push(node.host);
                }
            }
        }

        for process in self.process.iter() {
            if let Some(node) = process.object.node.as_ref() {
                if !hosts.contains(&node.host) {
                    nodes.push(node);
                    hosts.push(node.host);
                }              
            }
        }

        for store in self.store.iter() {
            if let Some(node) = store.object.node.as_ref() {
                if !hosts.contains(&node.host) {
                    nodes.push(node);
                    hosts.push(node.host);
                }              
            }
        }
 
        for object in self.object.iter() {
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
    pub fn get_mut_spawners<'a>(&'a mut self) -> impl Iterator<Item = (&'a mut (dyn Spawner + Send + 'a), String)> {
        let mut vec_objects = Vec::new();
        let mut vec_names = Vec::new();

        for object in self.object.iter_mut() {
            let name = object.name.clone();
            let spawner = object as _; 
            vec_objects.push(spawner);
            vec_names.push(name);
        }

        for store in self.store.iter_mut() {
            let name = store.object.name.clone();
            let spawner = store as _; 
            vec_objects.push(spawner);
            vec_names.push(name);
        }

        for process in self.process.iter_mut() {
            let name = process.object.name.clone();
            let spawner = process as _; 
            vec_objects.push(spawner);
            vec_names.push(name);
        }

        for generate in self.generate.iter_mut() {
            let name = generate.object.name.clone();
            let spawner = generate as _; 
            vec_objects.push(spawner);
            vec_names.push(name);
        }

        vec_objects.into_iter().zip(vec_names)
    }
}

impl Spawner for Config {
    fn build(&mut self) {
        thread::scope(|s| {
            let spawners = self.get_mut_spawners();
            for (spawner, name) in spawners {
                let _ = thread::Builder::new()
                    .name(name)
                    .spawn_scoped(s, || {
                        spawner.build();
                });
            }
        });
    }

    fn setup(&mut self) {
        thread::scope(|s| {
            let spawners = self.get_mut_spawners();
            for (spawner, name) in spawners {
                let _ = thread::Builder::new()
                    .name(name)
                    .spawn_scoped(s, || {
                        spawner.setup();
                });
            }
        });
    }

    fn deploy(&mut self) {
        thread::scope(|s| {
            let spawners = self.get_mut_spawners();
            for (spawner, name) in spawners {
                let _ = thread::Builder::new()
                    .name(name)
                    .spawn_scoped(s, || {
                        spawner.deploy();
                });
            }
        });
    }
}


