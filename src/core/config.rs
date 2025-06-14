use crate::shared::Spawner;

use super::{generate::Generate, object::Object, process::Process, store::Store, types::Node};

/// The config object. Contains all object types tasks
pub struct Config {
    pub api_port: u16,
    pub store: Vec<Store>,
    pub process: Vec<Process>,
    pub generate: Vec<Generate>,
    pub object: Vec<Object>,
}

impl Config {
    pub fn get_nodes<'a>(&'a self) -> Vec<&'a Node> {
        let mut nodes = Vec::new();
        let mut hosts = Vec::new();

        for generate in self.generate.iter() {
            if let Some(node) = generate.object.node.as_ref() {
                if !hosts.contains(&node.host) {
                    nodes.push(node);
                    hosts.push(node.host.clone());
                }
            }
        }

        for process in self.process.iter() {
            if let Some(node) = process.object.node.as_ref() {
                if !hosts.contains(&node.host) {
                    nodes.push(node);
                    hosts.push(node.host.clone());
                }              
            }
        }

        for store in self.store.iter() {
            if let Some(node) = store.object.node.as_ref() {
                if !hosts.contains(&node.host) {
                    nodes.push(node);
                    hosts.push(node.host.clone());
                }              
            }
        }
 
        for object in self.object.iter() {
            if let Some(node) = object.node.as_ref() {
                if !hosts.contains(&node.host) {
                    nodes.push(node);
                    hosts.push(node.host.clone());
                }               
            }
        }       

        nodes
    }

    pub fn get_mut_spawners<'a>(&'a mut self) -> impl Iterator<Item = (&'a mut (dyn Spawner + Send + 'a), String)> {
        let mut vec_objects = Vec::new();
        let mut vec_names = Vec::new();

        for object in self.object.iter_mut() {
            let name = object.name.clone();
            let spawner = object as &mut (dyn Spawner + Send); 
            vec_objects.push(spawner);
            vec_names.push(name);
        }

        for store in self.store.iter_mut() {
            let name = store.object.name.clone();
            let spawner = store as &mut (dyn Spawner + Send); 
            vec_objects.push(spawner);
            vec_names.push(name);
        }

        for process in self.process.iter_mut() {
            let name = process.object.name.clone();
            let spawner = process as &mut (dyn Spawner + Send); 
            vec_objects.push(spawner);
            vec_names.push(name);
        }

        for generate in self.generate.iter_mut() {
            let name = generate.object.name.clone();
            let spawner = generate as &mut (dyn Spawner + Send); 
            vec_objects.push(spawner);
            vec_names.push(name);
        }

        vec_objects.into_iter().zip(vec_names)
    }
}


