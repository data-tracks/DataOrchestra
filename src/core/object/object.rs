use std::collections::HashMap;
use std::path::Path;
use crate::core::adapters::docker::container::ContainerBuilder;
use crate::core::adapters::docker::{ComposeGroupBuilder, DockerManager};
use crate::core::adapters::ssh::Ssh;
use crate::core::adapters::{Container, ContainerType, Local, Runner, Uploader};
use crate::core::attach::attach_types::AttachType;
use crate::core::types::Data;
use crate::shared::{Address, Amount, Spawner};
use crate::core::types::Node;
use log::{error, info};

/// The `Object` type. Acts as a generic component. Implements basic fields that every object
/// should possess.
#[derive(Debug)]
pub struct Object {
    pub docker_group_builder: Option<ComposeGroupBuilder>,
    pub docker_container_builder: Option<ContainerBuilder>,
    pub docker_manager: Option<DockerManager>,
    pub start: Option<String>,
    pub node: Option<Node>,
    pub remote: Option<Address>,
    pub attach_type: Option<AttachType>,
    pub attach: Amount<Box<Object>>,
    pub ssh: Option<Ssh>,
    pub data: Vec<Data>,
    pub ansible: String
}

impl Default for Object {
    fn default() -> Self {
        Object { 
            docker_group_builder: None,
            docker_container_builder: None, 
            docker_manager: None,
            start: None, 
            node: None, 
            remote: None, 
            attach_type: None, 
            attach: Amount::None, 
            ssh: None,
            data: Vec::new(),
            ansible: "scripts/ansible/ansible-setup.yml".to_string()
        }
    }
}

impl Spawner for Object {
    fn build(&mut self) {
        
    }

    fn setup(&mut self) {
        
    }

    fn deploy(&mut self) {
        
    }
}

impl Object {
    pub fn start_ansible(&self) -> Result<(), String> {
        let script_path = "scripts/ansible/ansible-setup.yml";
        if !Path::new(script_path).is_file() {
            return Err(format!("Unable to find file {}", script_path)); 
        }

        if let Some(ref manager) = self.docker_manager {
            for container in manager.as_vec() {
                let command = format!("ansible-playbook {} -e \"port={}\"", script_path, container.get_ssh_port().unwrap());

                if let Some(ref node) = self.node {
                    let runner = Box::new(node.ssh.as_ref().unwrap().clone()) as Box<dyn Runner + Send>;
                    let result = runner.exec(command);
                    if let Err(error) = result {
                        error!("{}", error);
                    }
                }
                else {
                    let runner = Box::new(Local::new()) as Box<dyn Runner + Send>;
                    let result = runner.exec(command);
                    if let Err(error) = result {
                        error!("{}", error);
                    }
                }
            }
        }

        Ok(())
    }

    /// Start starting script on remote object
    pub fn start_script(&self) -> Result<(), String> {
        if let Some(ref manager) = self.docker_manager {
            if manager.amount() > 1 {
                for (container, data) in self.iter_combine_data() {
                    if let Some(ref ssh) = container.ssh {
                        info!("Starting {} for {}", data.start, container.config.name.as_ref().unwrap());
                        if data.start.ends_with(".sh") {
                            ssh.exec(format!("sh {}", data.start))?;
                        }
                        else {
                            ssh.exec(format!("{}", data.start))?;
                        }
                    }
                    else {
                        panic!("Ssh client unavailable");
                    }
                }
            }
            else {
                if let Some(container) = manager.get_container(){
                    for data in self.data.iter() {
                        if let Some(ref ssh) = container.ssh {
                            info!("Starting {} for {}", data.start, container.config.name.as_ref().unwrap());
                            if data.start.ends_with(".sh") {
                                ssh.exec(format!("sh {}", data.start))?;
                            }
                            else {
                                ssh.exec(format!("{}", data.start))?;
                            }
                        } 
                        else {
                            panic!("Ssh client unavailable");
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Combine containers managed by manager with [`Data`] to [`Iterator`] 
    ///
    /// If `data` is empty, returns an empty iterator.
    ///
    /// If there is only on container, all entries in `data` get combined with that
    /// specific `container`
    pub fn iter_combine_data<'a >(&'a self) -> impl Iterator<Item = (&'a Container, &'a Data)> {
        let mut vec_container = Vec::<&Container>::new();
        let mut vec_data = Vec::<&Data>::new();

        // Early return for when data contains nothing
        if self.data.len() == 0 {
            return vec_container.into_iter().zip(vec_data);
        }

        if let Some(ref manager) = self.docker_manager {
            if manager.amount() == 1 {
                let container = manager.get_container();
                if let Some(container) = container {
                    for d in self.data.iter() {
                        vec_container.push(container);
                        vec_data.push(d);
                    }
                }
            }
            else 
            {
                let mut mapped_all_containers = HashMap::<&String, &Container>::new();
                for (_, item) in manager.containers.iter() {
                    match item {
                        ContainerType::Compose(compose) => {
                            for container in compose.containers.iter() {
                                mapped_all_containers.insert(container.config.name.as_ref().unwrap(), container);
                            } 
                        }
                        ContainerType::Container(container) => {
                            mapped_all_containers.insert(container.config.name.as_ref().unwrap(), container);
                        }
                    }
                }

                for d in self.data.iter() {
                    if let Some(ref mut container) = mapped_all_containers.get(&d.name) {
                        vec_container.push(container);
                        vec_data.push(d);
                    }
                };
            }
        }

        vec_container.into_iter().zip(vec_data)
    }

    pub fn upload_data(&self) -> Result<(), String> {
        for (container, data) in self.iter_combine_data() {
            if let Some(ref ssh) = container.ssh {
                let path = Path::new(&data.path);
                if path.is_dir() {
                    ssh.upload_directory(path, &data.destination)?;
                }
                else if path.is_file() {
                    ssh.upload_file(path, &data.destination)?;
                }

                if let Some(ref dependency) = data.dependency {
                    let dependency_path= Path::new(dependency);
                    ssh.exec("mkdir /scripts".to_string())?;
                    ssh.upload_file(dependency_path, Path::new(&format!("/scripts/{}", &dependency_path.file_name().unwrap().to_str().unwrap())))?;
                    ssh.exec(format!("sh /scripts/{}", dependency_path.file_name().unwrap().to_str().unwrap()))?;
                }
            } 
            else {
                panic!("Ssh client unavailable for container {} even though upload data was specified", container.config.name.as_ref().unwrap());
            }
        }

        Ok(())
    }
}
