use std::{path::Path};
use std::thread::{self, JoinHandle};

use crate::core::adapters::docker::ContainerType;
use crate::core::adapters::ssh::Ssh;
use crate::core::attach::attach_types::AttachType;
use crate::shared::traits::Start;
use crate::shared::{Address, Amount, Node};

/// The `Object` type. Acts as a generic component. Implements basic fields that every object
/// should possess.
#[derive(Debug)]
pub struct Object {
    pub docker: Option<ContainerType>,
    pub start: Option<String>,
    pub data: Option<String>,
    pub node: Option<Node>,
    pub remote: Option<Address>,
    pub attach_type: Option<AttachType>,
    pub attach: Amount<Box<Object>>,
    pub ssh: Option<Ssh>
}

impl Default for Object {
    fn default() -> Self {
        Object { 
            docker: None, 
            start: None, 
            data: None, 
            node: None, 
            remote: None, 
            attach_type: None, 
            attach: Amount::None, 
            ssh: None 
        }
    }
}

impl Start<()> for Object {
    fn start(self) -> JoinHandle<()> {
        thread::Builder::new().name("object".to_string()).spawn(move || {
        
        }).unwrap()
    }
} 

impl Object {
    pub fn upload_data(&self) -> String {
        let mut upload_directory = String::from("/");
        if let Some(ref data) = self.data {
            let upload = self.ssh.as_ref().unwrap().upload_directory(&Path::new(&data), &Path::new("/"));
            if let Ok(dir) = upload {
                upload_directory = dir;
            }
        }

        upload_directory
    }

    
}
