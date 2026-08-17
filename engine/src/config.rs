use super::{object::Object, types::Node};
use crate::adapters::agent::Agent;
use crate::state::State;
use crate::traits::Spawnable;
use std::thread;

/// The config object. Contains all object types tasks
#[derive(Debug, Default)]
pub struct Config {
    pub objects: Vec<Object>,
    pub agents: Vec<Agent>,
}


impl Config {
    pub fn get_objects(&self) -> Vec<&Object> {
        self.objects.iter().collect()
    }

    pub fn get_objects_mut(&mut self) -> Vec<&mut Object> {
        self.objects.iter_mut().collect()
    }

    /// Combine two [Config] into a single configuration
    pub fn combine(&mut self, other: Config) {
        self.objects.extend(other.objects);

        for other_agent in other.agents.into_iter() {
            let other_node = other_agent.object.node.as_ref().unwrap();
            let exists = self.agents.iter().any(|agent| {
                agent
                    .object
                    .node
                    .as_ref()
                    .is_some_and(|node| node.host.eq(&other_node.host))
            });

            if !exists {
                self.agents.push(other_agent);
            }
        }
    }

    /// Get mutable reference from all nodes from all object types
    pub fn get_object_nodes_mut(&mut self) -> Vec<&mut Node> {
        let mut nodes = Vec::new();

        for object in self.get_objects_mut().into_iter() {
            if let Some(node) = object.node.as_mut() {
                nodes.push(node);
            }
        }

        nodes
    }

    /// Get reference from all unique nodes from all object types
    pub fn get_unique_nodes(&self) -> Vec<&Node> {
        let mut nodes = Vec::new();
        let mut hosts = Vec::new();

        for object in self.get_objects().into_iter() {
            if let Some(node) = object.node.as_ref() && !hosts.contains(&node.host){
                nodes.push(node);
                hosts.push(node.host);
            }
        }

        nodes
    }

    pub fn get_spawners<'a>(
        &'a self,
    ) -> impl Iterator<Item = (&'a (dyn Spawnable + Send + 'a), String)> {
        let mut vec_objects: Vec<(&'a (dyn Spawnable + Send + 'a), String)> = Vec::new();

        for object in self.objects.iter() {
            let name = object.name.clone();
            let spawner = object as _;
            vec_objects.push((spawner, name));
        }

        vec_objects.into_iter()
    }

    /// Get mutable reference to all objects in config which implement the [`Spawnable`] trait.
    pub fn get_mut_spawners<'a>(
        &'a mut self,
    ) -> impl Iterator<Item = (&'a mut (dyn Spawnable + Send + 'a), String)> {
        let mut vec_objects: Vec<(&'a mut (dyn Spawnable + Send + 'a), String)> = Vec::new();

        for object in self.objects.iter_mut() {
            let name = object.name.clone();
            let spawner = object as _;
            vec_objects.push((spawner, name));
        }

        vec_objects.into_iter()
    }

    pub fn get_number_of_components(&self) -> usize {
        self.get_spawners().count()
    }
}

impl Spawnable for Config {
    fn name(&self) -> String {
        "config".to_string()
    }

    fn state(&self) -> State {
        for (spawner, _) in self.get_spawners() {
            if spawner.state().is_not_running() {
                return State::NotRunning;
            }
        }

        State::Running
    }

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
