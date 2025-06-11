use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::path::Path;
use crate::core::adapters::docker::container::ContainerBuilder;
use crate::core::adapters::docker::ComposeGroupBuilder;
use crate::core::adapters::ssh::Ssh;
use crate::core::adapters::{Container, ContainerType, Local, Run, Runner, Uploader};
use crate::core::types::data::NodeData;
use crate::core::types::data::DockerData;
use crate::shared::{Spawner, ARGS};
use crate::core::types::Node;
use log::{debug, error, info, warn};

/// The `Object` type. Acts as a generic component. Implements basic fields that every object
/// should possess.
#[derive(Debug)]
pub struct Object {
    // Docker builder for compose 
    pub docker_group_builder: Option<ComposeGroupBuilder>,
    // Docker builder for container
    pub docker_container_builder: Option<ContainerBuilder>,
    // Docker manager. Manages containers for its object
    pub docker_manager: ContainerType,
    // Node connection
    pub node: Option<Node>,
    // Ssh connection to object location
    pub ssh: Option<Ssh>,
    // Data to be uploaded to node 
    pub node_data: Vec<NodeData>,
    // Data to be uploaded to docker container 
    pub docker_data: Vec<DockerData>,
    // Ansible script responsible for the setup of the enviroment
    pub ansible: String,
}

impl Default for Object {
    fn default() -> Self {
        Object { 
            docker_group_builder: None,
            docker_container_builder: None, 
            docker_manager: ContainerType::Empty,
            node: None, 
            ssh: None,
            node_data: Vec::new(),
            docker_data: Vec::new(),
            ansible: "scripts/ansible/ansible-setup.yml".to_string(),
        }
    }
}

impl Spawner for Object {
    /// Building of Object. After invocation:
    /// - Base variables set for systems
    /// - Node ssh connection available
    /// - Node data uploaded
    fn build(&mut self) {
        if let Some(ref mut node) = self.node {
            if let Some(key) = ARGS.get().as_ref().unwrap().ssh_key.as_ref() {
                let result = node.set_ssh(key);
                if let Err(error) = result {
                    panic!("Unable to setup ssh for {} {}", node.host, error);
                }
            }
            else {
                panic!("No ssh key provided for node");
            }
        }

        // Take ownership of ComposeGroupBuilder out of object to prevent partial move 
        if let Some(group) = self.docker_group_builder.take() {
            info!("Setting up docker compose");
            let mut compose = group.build();
            // If a node was specified, docker compose file needs to be uploaded to node
            if let Some(ref node) = self.node {
                if let Some(ref ssh) = node.ssh {
                    let runner = ssh.to_box_runner();
                    compose.runner = runner;

                    let result = ssh.exec("mkdir docker/".to_string());
                    if let Err(error) = result {
                        error!("Unable to create docker/ folder | {}", error);
                    }
                    let local_path = compose.compose.clone().unwrap(); 
                    if let Some(file_name) = Path::new(compose.compose.as_ref().unwrap())
                            .file_name()
                            .and_then(|name| name.to_str()) 
                    {
                        compose.compose = Some(format!("docker/{}", file_name));
                    }
                    let result = ssh.upload_file(local_path, compose.compose.as_ref().unwrap());
                    if let Err(error) = result {
                        panic!("Unable to upload compose file {}", error);
                    }

                    let result = ssh.upload_directory("images/", "docker/");
                    if let Err(error) = result {
                        error!("{}", error);
                    }
                }
                else {
                    panic!("No runner available for docker compose. Node was provided but ssh connection was not properly loaded");
                }
            }
            self.docker_manager = ContainerType::Compose(compose);
        }
        // Take ownership of ContainerBuilder out of object to prevent partial move
        else if let Some(container) = self.docker_container_builder.take() {
            info!("Setting up docker container");
            let mut container = container.build();
            // If a node was specified, dockerfile needs to be uploaded to node
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
            self.docker_manager = ContainerType::Container(container);
        }
        else {
            panic!("No Docker Builder available");
        }

        // Upload all node data to relevant node. This is done before the setup as the
        // specialization setup may start before object setup, thus data could be missing
        debug!("Uploading node data");
        if let Some(node) = self.node.as_ref() {
            if let Some(ssh) = node.ssh.as_ref() {
                for data in self.node_data.iter() {
                    let path = Path::new(&data.path);
                    if path.is_dir() {
                        let result = ssh.upload_directory(path, &data.destination);
                        if let Err(error) = result {
                            error!("{}", error);
                        }
                    }
                    else if path.is_file() {
                        let result = ssh.upload_file(path, &data.destination);
                        if let Err(error) = result {
                            error!("{}", error);
                        }
                    }
                } 
            }
        }
    }

    /// Setup of object. After invocation:
    /// - Docker container functionally running
    /// - Docker container configured with base libraries and ssh connection
    /// - Docker container data uploaded
    fn setup(&mut self) {
        let result = self.docker_manager.run();
        if let Err(error) = result {
            error!("{}", error);
        }

        // Setup ssh connection for all containers
        match &mut self.docker_manager { 
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
            ContainerType::Empty => {
                panic!("No docker container or compose available");
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

    /// Deployement of Object. After invocation:
    /// - Start script running
    fn deploy(&mut self) {
        let result = self.start_script();
        if let Err(error) = result {
            error!("{}", error);
        }
    }
}

impl Object {
    pub fn start_ansible(&self) -> Result<(), String> {
        if !Path::new(&self.ansible).is_file() {
            return Err(format!("Unable to find file {}", &self.ansible)); 
        }

        for container in self.docker_manager.containers_ref_vec() {
            let ssh_port = container.get_ssh_port();
            if let Some(port) = ssh_port {
                let mut command = format!("ansible-playbook {} -e \"port={}\"", &self.ansible, port);

                if let Some(ref node) = self.node {
                    command = format!("ansible-playbook {} -e \"port={}\" -e \"host={}\"", &self.ansible, port, node.host);
                }    

                let runner = Local::new();
                runner.exec(command)?;
            }
            else {
                warn!("{}", format!("No ssh port available for {}. Unable to Configure with ansible", container.config.name.as_ref().unwrap()));
            }
        }

        Ok(())
    }

    /// Start starting script on remote object
    pub fn start_script(&self) -> Result<(), String> {
        for (container, data) in Self::iter_combine_data(&self.docker_manager.containers_ref_vec(), &self.docker_data) {
            if let Some(ref ssh) = container.ssh {
                info!("Starting {} for {}", data.start, container.config.name.as_ref().unwrap());

                let result = ssh.exec(format!("test -f {} && echo \"ok\" || echo \"err\"", data.start));
                if let Err(error) = result {
                    error!("{}", error);
                }
                else if let Ok(result) = result {
                    if result.eq("err") {
                        error!("Unable to find file {}. Check if the path is correctly formatted", data.start);
                    }
                }


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
            
        Ok(())
    }

    /// Combine containers managed by manager with [`Data`] to [`Iterator`] 
    ///
    /// If `data` is empty, returns an empty iterator.
    ///
    /// If there is only on container, all entries in `data` get combined with that
    /// specific `container`
    pub fn iter_combine_data<'a >(containers: &'a Vec<&'a Container>, data: &'a Vec<DockerData>) -> impl Iterator<Item = (&'a Container, &'a DockerData)> {
        let mut vec_container = Vec::<&Container>::new();
        let mut vec_data = Vec::<&DockerData>::new();

        // Early return for when data contains nothing
        if data.len() == 0 {
            return vec_container.into_iter().zip(vec_data);
        }

        // Map all data entries to specific containers
        if containers.len() == 1 {
            let container = containers.get(0);
            if let Some(container) = container {
                for d in data.iter() {
                    vec_container.push(container);
                    vec_data.push(d);
                }
            }
        }
        // Map data entries according to the container names
        else 
        {
            let mut mapped_all_containers = HashMap::<&String, &Container>::new();
            for container in containers.iter() {
                mapped_all_containers.insert(container.config.name.as_ref().unwrap(), container);
            }

            for d in data.iter() {
                if let Some(container) = mapped_all_containers.get(&d.name) {
                    vec_container.push(container);
                    vec_data.push(d);
                }
            };
        }

        vec_container.into_iter().zip(vec_data)
    }


    /// Upload all data specified in the data field of the [`Object`] to docker container
    pub fn upload_data(&self) -> Result<(), String> {
        for (container, data) in Self::iter_combine_data(&self.docker_manager.containers_ref_vec(), &self.docker_data) {
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
