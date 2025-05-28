use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::path::Path;
use std::process::exit;
use crate::core::adapters::docker::container::ContainerBuilder;
use crate::core::adapters::docker::{self, ComposeGroupBuilder, DockerManager};
use crate::core::adapters::ssh::Ssh;
use crate::core::adapters::{Container, ContainerType, Local, Run, Runner, Uploader};
use crate::core::attach::attach_types::AttachType;
use crate::core::types::Data;
use crate::shared::{Address, Amount, Spawner};
use crate::core::types::Node;
use log::{debug, error, info, warn};

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
        let mut manager = DockerManager::new();

        if let Some(ref mut node) = self.node {
            let result = node.load_ssh();
            if let Err(error) = result {
                panic!("Unable to setup ssh for {} {}", node.host, error);
            }
        }

        // Take ownership of ComposeGroupBuilder out of object to prevent partial move 
        if let Some(group) = self.docker_group_builder.take() {
            info!("Setting up docker compose");
            let mut compose = group.build();
            if let Some(ref node) = self.node {
                if let Some(ref ssh) = node.ssh {
                    let runner = ssh.to_box_runner();
                    compose.runner = runner;

                    ssh.exec("mkdir docker/".to_string());
                    let local_path = compose.compose.clone().unwrap(); 
                    if let Some(file_name) = Path::new(compose.compose.as_ref().unwrap())
                            .file_name()
                            .and_then(|name| name.to_str()) 
                    {
                        compose.compose = Some(format!("docker/{}", file_name));
                    }
                    let result = ssh.upload_file(local_path, compose.compose.as_ref().unwrap());
                    if let Err(error) = result {
                        panic!("Unable to upload dockerfile {}", error);
                    }
                }
                else {
                    panic!("No runner available for docker compose. Node was provided but ssh connection was not properly loaded");
                }
            }
            manager.add("compose", ContainerType::Compose(compose));
        }
        // Take ownership of ContainerBuilder out of object to prevent partial move
        else if let Some(container) = self.docker_container_builder.take() {
            info!("Setting up docker container");
            let mut container = container.build();

            if let Some(ref node) = self.node {
                if let Some(ref ssh) = node.ssh {
                    let runner = ssh.to_box_runner();
                    container.runner = runner;
    
                    if let Some(dockerfile) = container.source.dockerfile.as_mut() {
                        let result = ssh.exec("mkdir docker/".to_string());
                        if let Err(error) = result {
                            error!("Unable to create docker/ folder on node | {}", error);
                        }

                        let local_path = dockerfile.clone(); 
                        if let Some(file_name) = Path::new(dockerfile)
                                .file_name()
                                .and_then(|name| name.to_str()) 
                        {
                            *dockerfile = format!("docker/{}", file_name);
                        }
                        let result = ssh.upload_file(local_path, dockerfile);
                        if let Err(error) = result {
                            panic!("Unable to upload dockerfile {}", error);
                        }

                    }
                }
            }
            manager.add(container.config.name.clone().unwrap(), ContainerType::Container(container));
        }
        else {
            panic!("No Docker Builder available");
        }

        self.docker_manager = Some(manager);
    }

    fn setup(&mut self) {
        let result = self.start_containers();
        if let Err(error) = result {
            error!("{}", error);
        }

        if let Some(ref mut manager) = self.docker_manager {
            for (_, item) in manager.containers.iter_mut() {
                match item { 
                    ContainerType::Compose(compose) => {
                        for container in compose.containers.iter_mut() {
                            if let Some(ref node) = self.node {
                                if container.get_ssh_port().is_some() {
                                    let result = container.load_ssh(node.host);
                                    if let Err(error) = result {
                                        error!("Unable to load ssh connection for {} {}", node.host, error);
                                    }
                                }
                            }
                            else {
                                let result = container.load_ssh(IpAddr::V4(Ipv4Addr::LOCALHOST));
                                if let Err(error) = result {
                                    error!("Unable to load ssh connection for localhost {}", error);
                                }  
                            }
                        }
                    }
                    ContainerType::Container(container ) => {
                        if let Some(ref node) = self.node {
                            if container.get_ssh_port().is_some() {
                                let result = container.load_ssh(node.host);
                                if let Err(error) = result {
                                    error!("Unable to load ssh connection for {} {}", node.host, error);
                                }
                            }
                        }
                        else {
                            let result = container.load_ssh(IpAddr::V4(Ipv4Addr::LOCALHOST));
                            if let Err(error) = result {
                                error!("Unable to load ssh connection for localhost {}", error);
                            } 
                        }
                    }
                }
            }
        }

        // No panic as ansible depends on a ssh port, where some compose files may not provide
        // these
        let result = self.start_ansible();
        if let Err(error) = result {
            error!("Unable to start ansible {}", error);
        }

        // Upload data to docker containers
        let result = self.upload_data();
        if let Err(error) = result {
            panic!("Unable to upload data {}", error);
        }
    }

    fn deploy(&mut self) {
        let result = self.start_script();
        if let Err(error) = result {
            error!("{}", error);
        }
    }
}

impl Object {
    pub fn start_containers(&mut self) -> Result<(), String> {
        // Start containers and move to manager
        if let Some(ref mut manager) = self.docker_manager {
            for (name, item) in manager.containers.iter_mut() {
                debug!("Running docker {}", name);
                item.run()?;
            }
        }

        Ok(())
    }

    pub fn start_ansible(&self) -> Result<(), String> {
        let script_path = "scripts/ansible/ansible-setup.yml";
        if !Path::new(script_path).is_file() {
            return Err(format!("Unable to find file {}", script_path)); 
        }

        if let Some(ref manager) = self.docker_manager {
            for container in manager.as_vec() {
                let ssh_port = container.get_ssh_port();
                if let Some(port) = ssh_port {
                    let mut command = format!("ansible-playbook {} -e \"port={}\"", script_path, port);

                    if let Some(ref node) = self.node {
                        command = format!("ansible-playbook {} -e \"port={}\" -e \"host={}\"", script_path, port, node.host);
                    }    
                    /*
                        if let Some(ref ssh) = node.ssh {
                            let runner = ssh.to_box_runner();
                            runner.exec(command)?;
                        }
                    }
                    else {
                        let runner = Local::new().to_box_runner();
                        runner.exec(command)?;
                    }
                    */

                    let runner = Local::new().to_box_runner();
                    runner.exec(command)?;
                }
                else {
                    warn!("{}", format!("No ssh port available for {}. Unable to Configure with ansible", container.config.name.as_ref().unwrap()));
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

                        let result = ssh.exec(format!("test -f {} && echo \"ok\" || echo \"err\"", data.start));
                        debug!("{:?}", &result);

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

                            let result = ssh.exec(format!("test -f {} && echo \"ok\" || echo \"err\"", data.start));
                            debug!("{:?}", &result);

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
