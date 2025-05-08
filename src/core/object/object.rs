use std::thread::{self, JoinHandle};
use crate::core::adapters::docker::container::ContainerBuilder;
use crate::core::adapters::docker::ComposeGroupBuilder;
use crate::core::adapters::ssh::Ssh;
use crate::core::attach::attach_types::AttachType;
use crate::core::types::Data;
use crate::shared::traits::Start;
use crate::shared::{Address, Amount};
use crate::core::types::Node;

/// The `Object` type. Acts as a generic component. Implements basic fields that every object
/// should possess.
#[derive(Debug)]
pub struct Object {
    pub docker_group_builder: Option<ComposeGroupBuilder>,
    pub docker_container_builder: Option<ContainerBuilder>,
    pub start: Option<String>,
    pub node: Option<Node>,
    pub remote: Option<Address>,
    pub attach_type: Option<AttachType>,
    pub attach: Amount<Box<Object>>,
    pub ssh: Option<Ssh>,
    pub data: Vec<Data>,
}

impl Default for Object {
    fn default() -> Self {
        Object { 
            docker_group_builder: None,
            docker_container_builder: None, 
            start: None, 
            node: None, 
            remote: None, 
            attach_type: None, 
            attach: Amount::None, 
            ssh: None,
            data: Vec::new(),
        }
    }
}

impl Start<()> for Object {
    fn start(self) -> JoinHandle<()> {
        thread::Builder::new().name("object".to_string()).spawn(move || {
        
        }).unwrap()
    }
} 
