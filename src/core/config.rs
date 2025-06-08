use super::{generate::Generate, object::Object, process::Process, store::Store, types::Node};

pub struct Config {
    pub store: Vec<Store>,
    pub process: Vec<Process>,
    pub generate: Vec<Generate>,
    pub object: Vec<Object>,
}

impl Config {
    pub fn get_all_nodes<'a>(&'a self) -> Vec<&'a Node> {
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
}


