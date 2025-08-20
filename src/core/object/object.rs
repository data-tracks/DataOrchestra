use std::collections::HashMap;
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr};
use std::path::Path;
use std::time::Duration;
use crate::core::adapters::{ping, ComposeBuilder, Container, ContainerBuilder, ContainerType, Local, Rsync, RsyncBuilder, Run, Runner, Uploader};
use crate::core::types::data::{Data, DataTypes, GetData, VolatileData};
use crate::shared::{repeat_on_err, repeat_on_err_mut};
use crate::log_time;
use crate::core::types::{Executables, GetExecutables, Node, Script};
use derive_builder::Builder;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use crate::core::traits::Spawner;

/// Represents connection in the distributed system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    /// Where data from the object goes to
    #[serde(default)]
    pub to: Vec<String>,
    /// If object should be ignored when parsing to the graph structure
    #[serde(default)]
    pub ignore: bool
}

impl Default for Graph {
    fn default() -> Self {
        Graph
        {
            to: Vec::new(),
            ignore: false
        }
    }
}

/// The `Object` type. Acts as a generic component. Implements basic fields that every object
/// should possess.
#[derive(Debug, Builder)]
#[builder(pattern = "owned")]
pub struct Object {
    // Name of object. Default is the object type itself
    #[builder(setter(into), default)]
    pub name: String,
    // Data related to graph
    #[builder(default)]
    pub graph: Graph,
    // Docker builder for compose 
    #[builder(setter(strip_option), default)]
    pub docker_group_builder: Option<ComposeBuilder>,
    // Docker builder for container
    #[builder(setter(strip_option), default)]
    pub docker_container_builder: Option<ContainerBuilder>,
    // Docker manager. Manages containers for its object
    #[builder(default)]
    pub docker_manager: ContainerType,
    // Node connection
    #[builder(setter(strip_option), default)]
    pub node: Option<Node>,
    #[builder(setter(each = "resource"), default)] 
    pub resources: Vec<DataTypes>,
    #[builder(setter(each = "executable"), default)]
    pub executables: Vec<Executables>,
    // Ansible script responsible for the setup of the environment
    #[builder(setter(into), default = "default_ansible()")]
    pub ansible: String,
    #[builder(default = "default_runner()")]
    pub runner: Box<dyn Runner + Send + Sync>,
    pub uploader: Option<Box<dyn Uploader + Send + Sync>>
}

pub fn default_ansible() -> String {
    "scripts/ansible/ansible-setup.yml".to_string()
}

pub fn default_runner() -> Box<dyn Runner + Send + Sync> {
    Box::new(Local::new())
}

impl ObjectBuilder {
    pub fn ignore_graph(mut self, ignore_graph: bool) -> Self {
        let graph = self.graph.get_or_insert_default();
        graph.ignore = ignore_graph;
        self
    }

    pub fn node_data(self, data: Data) -> Self {
        self.resource(DataTypes::NodeData(data))
    }

    pub fn docker_data(self, data: Data) -> Self {
        self.resource(DataTypes::DockerData(data))
    }

    pub fn volatile_node_data(self, data: VolatileData) -> Self {
        self.resource(DataTypes::VolatileNodeData(data))
    }

    pub fn volatile_docker_data(self, data: VolatileData) -> Self {
        self.resource(DataTypes::VolatileDockerData(data))
    }

    pub fn script(self, script: Script) -> Self {
        self.executable(Executables::Script(script))
    }
}


impl Default for Object {
    fn default() -> Self {
        Object { 
            name: "object".to_string(),
            graph: Graph::default(),
            docker_group_builder: None,
            docker_container_builder: None, 
            docker_manager: ContainerType::Empty,
            node: None, 
            resources: Vec::new(),
            executables: Vec::new(),
            ansible: default_ansible(),
            runner: default_runner(),
            uploader: None
        }
    }
}

impl Spawner for Object {
    /// Building of Object. After invocation:
    /// - Base variables set for systems
    /// - Node ssh connection available
    /// - Node data uploaded
    fn build(&mut self) {
        info!("Building {}", self.name);

        // Take ownership of ComposeGroupBuilder out of object to prevent partial move 
        if let Some(group) = self.docker_group_builder.take() {
            info!("Setting up docker compose");
            let mut compose = group
                .build()
                .expect("Unable to build compose");
            // If a node was specified, docker compose file needs to be uploaded to node
            if let Some(ref node) = self.node {
                if let Some(ref ssh) = node.ssh {
                    let runner = ssh.to_box_runner();
                    compose.runner = runner;

                    let result = ssh.exec("mkdir docker/".to_string());
                    if let Err(error) = result {
                        error!("Unable to create docker/ folder | {error}");
                    }
                    let local_path = compose.config.compose.clone().unwrap(); 
                    if let Some(file_name) = Path::new(compose.config.compose.as_ref().unwrap())
                            .file_name()
                            .and_then(|name| name.to_str()) 
                    {
                        compose.config.compose = Some(format!("docker/{file_name}"));
                    }
                    let result = ssh.upload_file(local_path.as_ref(), compose.config.compose.as_ref().unwrap().as_ref());
                    if let Err(error) = result {
                        panic!("Unable to upload compose file {error}");
                    }

                    let result = ssh.upload_directory("images/".as_ref(), "docker/".as_ref());
                    if let Err(error) = result {
                        error!("{error}");
                    }
                }
                else {
                    panic!("No runner available for docker compose. Node was provided but ssh connection was not properly loaded");
                }
            }
            self.docker_manager = ContainerType::Compose(compose);
        }
        // Take ownership of ContainerBuilder out of object to prevent partial move
        else if let Some(mut container) = self.docker_container_builder.take() {
            info!("Setting up docker container");
            let mut container = container
                .publish(22)
                .build()
                .expect("Unable to build container");

            // If a node was specified, dockerfile needs to be uploaded to node
            if let Some(ref node) = self.node {
                if let Some(ref ssh) = node.ssh {
                    let runner = ssh.to_box_runner();
                    container.runner = runner;
                    if let Some(dockerfile) = container.config.dockerfile.as_mut() {
                        let result = ssh.exec("mkdir docker/".to_string());
                        if let Err(error) = result {
                            error!("Unable to create docker/ folder on node ({error})");
                        }

                        let local_path = dockerfile.clone(); 
                        if let Some(file_name) = Path::new(dockerfile)
                                .file_name()
                                .and_then(|name| name.to_str()) 
                        {
                            *dockerfile = format!("docker/{file_name}");
                        }
                        let result = ssh.upload_file(local_path.as_ref(), dockerfile.as_ref());
                        if let Err(error) = result {
                            panic!("Unable to upload dockerfile ({error})");
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
        if let Some(node) = self.node.as_ref() && let Some(ssh) = node.ssh.as_ref(){
            for data in self.resources.get_node_data() {
                let path = Path::new(&data.source);
                if path.is_dir() {
                    let result = ssh.upload_directory(path, data.destination.as_ref());
                    if let Err(error) = result {
                        error!("{error}");
                    }
                }
                else if path.is_file() {
                    let result = ssh.upload_file(path, data.destination.as_ref());
                    if let Err(error) = result {
                        error!("{error}");
                    }
                }
            }
        }

        info!("Finished building {}", self.name);
    }

    /// Setup of object. After invocation:
    /// - Docker container functionally running
    /// - Docker container configured with base libraries and ssh connection
    /// - Docker container data uploaded
    fn setup(&mut self) {
        info!("Setting up {}", self.name);

        log_time!("Starting containers");
        let result = self.docker_manager.run();
        if let Err(error) = result {
            error!("{error}");
        }
        log_time!("Finished starting containers");

        log_time!("Starting polling ssh port containers");
        for container in self.docker_manager.containers_ref_vec() {
            let ssh_port = container.get_ssh_port();
            if let Some(ssh_port) = ssh_port {
                let host: &IpAddr;

                if let Some(node) = self.node.as_ref() {
                    host = &node.host;
                }
                else {
                    host = &IpAddr::V4(Ipv4Addr::LOCALHOST);
                }

                debug!("Polling docker container ssh connection ({host}:{ssh_port})");
                
                let result = repeat_on_err(|| { 
                    ping(host, ssh_port)
                }, 5, Some(Duration::from_secs(1)));
                if let Err(error) = result {
                    panic!("{error}");
                }
            }
        }
        log_time!("Finished polling ssh port containers");

        // Setup ssh connection for all containers
        log_time!("Start container ssh sessions");
        match &mut self.docker_manager { 
            ContainerType::Compose(compose) => {
                for container in compose.containers.iter_mut() {
                    if let Some(ref node) = self.node {
                        if container.get_ssh_port().is_some() {
                            debug!("Loading ssh session for container");

                            let result = repeat_on_err_mut(|| {
                                container.load_ssh(node.host)
                            }, 5, Some(Duration::from_secs(1)));
                            if let Err(error) = result {
                                error!("({}) ({error})", node.host);
                            }
                        }
                    }
                    else {
                        let result = repeat_on_err_mut(|| {
                            container.load_ssh(IpAddr::V4(Ipv4Addr::LOCALHOST))
                        }, 10, Some(Duration::from_secs(2)));
                        if let Err(error) = result {
                            error!("Unable to load ssh connection for local container ({error})");
                        } 
                    }
                }
            }
            ContainerType::Container(container ) => {
                if let Some(ref node) = self.node {
                    if container.get_ssh_port().is_some() {
                        debug!("Loading ssh session for container");
                        let result = repeat_on_err_mut(|| {
                            container.load_ssh(node.host)
                        }, 10, Some(Duration::from_secs(2)));
                        if let Err(error) = result {
                            error!("({}) ({error})", node.host);
                        }
                    }
                }
                else {
                    let result = repeat_on_err_mut(|| {
                        container.load_ssh(IpAddr::V4(Ipv4Addr::LOCALHOST))
                    }, 5, Some(Duration::from_secs(1)));
                    if let Err(error) = result {
                        error!("Unable to load ssh connection for local container ({error})");
                    } 
                }
            }
            ContainerType::Empty => {
                panic!("No docker container or compose available");
            }
        }
        log_time!("Finished container ssh sessions");

        // No panic as ansible depends on a ssh port, where some compose files may not provide
        // these
        log_time!("Start ansible");
        let result = self.start_ansible();
        if let Err(error) = result {
            error!("Unable to start ansible ({error})");
        }
        log_time!("Finished ansible");

        // Upload data to docker containers
        log_time!("Start uploading data");
        let result = self.upload_data();
        if let Err(error) = result {
            panic!("Unable to upload data ({error})");
        }
        log_time!("Finished uploading data");

        info!("Finished setting up {}", self.name);
    }

    /// Deployement of Object. After invocation:
    /// - Start script running
    fn deploy(&mut self) {
        info!("Deploying {}", self.name);

        let result = self.start_script();
        if let Err(error) = result {
            error!("Unable to start script ({error})");
        }

        info!("Finished deploying {}", self.name);
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
                let result = runner.exec(command)
                    .map_err(|err| err.to_string());
                if let Err(error) = result {
                    if !error.contains("WARNING") {
                        error!("{error}");
                    }
                }
            }
            else {
                warn!("No ssh port available for {}. Unable to Configure with ansible", container.config.name.as_ref().unwrap());
            }
        }

        Ok(())
    }

    /// Start starting script on remote object
    pub fn start_script(&self) -> Result<(), String> {
        for (container, script) in Self::iter_combine_script(&self.docker_manager.containers_ref_vec(), self.executables.get_scripts()) {
            if let Some(ref ssh) = container.ssh {
                info!("Starting {} for {}", script.path, container.config.name.as_ref().unwrap());

                let result = ssh.exec(format!("test -f {} && echo \"ok\" || echo \"err\"", script.path));
                if let Err(error) = result {
                    error!("{error}");
                }
                else if let Ok(result) = result {
                    if result.eq("err") {
                        error!("Unable to find file {}. Check if the path is correctly formatted", script.path);
                    }
                }

                if script.path.ends_with(".sh") {
                    ssh.exec(format!("sh {}", script.path))?;
                }
                else {
                    ssh.exec(script.path.to_string())?;
                }
            } 
            else {
                panic!("Ssh client unavailable");
            }
        }
            
        Ok(())
    }

    /// Combine containers with [`DockerData`] to [`Iterator`] 
    ///
    /// If `data` is empty, returns an empty iterator.
    ///
    /// If there is only on container, all entries in `data` get combined with that
    /// specific `container`
    pub fn iter_combine_script<'a >(containers: &'a Vec<&'a Container>, script: Vec<&'a Script>) -> impl Iterator<Item = (&'a Container, &'a Script)> {
        let mut vec_container = Vec::new();
        let mut vec_script = Vec::new();

        // Early return for when data contains nothing
        if script.is_empty() {
            return vec_container.into_iter().zip(vec_script);
        }

        // Map all data entries to specific containers
        if containers.len() == 1 {
            let container = containers.first();
            if let Some(container) = container {
                for d in script.iter() {
                    vec_container.push(container);
                    vec_script.push(d);
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

            for s in script.iter() {
                if let Some(name) = s.name.as_ref() {
                    if let Some(container) = mapped_all_containers.get(&name) {
                        vec_container.push(container);
                        vec_script.push(s);
                    }
                    else {
                        error!("Couldn't find docker container {name} for docker data");
                    }
                }
                else {
                    error!("Multiple docker containers found but docker data does not have name to specific docker container. Cannot upload docker data.")
                }
                
            };
        }

        vec_container.into_iter().zip(vec_script)
    }

    /// Combine containers with [`DockerData`] to [`Iterator`] 
    ///
    /// If `data` is empty, returns an empty iterator.
    ///
    /// If there is only on container, all entries in `data` get combined with that
    /// specific `container`
    pub fn iter_combine_data<'a >(containers: &'a Vec<&'a Container>, data: Vec<&'a Data>) -> impl Iterator<Item = (&'a Container, &'a Data)> {
        let mut vec_container = Vec::new();
        let mut vec_data = Vec::new();

        // Early return for when data contains nothing
        if data.is_empty() {
            return vec_container.into_iter().zip(vec_data);
        }

        // Map all data entries to specific containers
        if containers.len() == 1 {
            let container = containers.first();
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
                if let Some(name) = d.name.as_ref() {
                    if let Some(container) = mapped_all_containers.get(name) {
                        vec_container.push(container);
                        vec_data.push(d);
                    }
                    else {
                        error!("Couldn't find docker container {name} for docker data");
                    }
                }
                else {
                    error!("Multiple docker containers found but docker data does not have name to specific docker container. Cannot upload docker data.")
                }
                
            };
        }

        vec_container.into_iter().zip(vec_data)
    }

    /// Combine containers with [`DockerSFTPData`] to [`Iterator`] 
    ///
    /// If `data` is empty, returns an empty iterator.
    ///
    /// If there is only on container, all entries in `data` get combined with that
    /// specific `container`
    pub fn iter_combine_sftp_data<'a >(containers: &'a Vec<&'a Container>, data: Vec<&'a VolatileData>) -> impl Iterator<Item = (&'a Container, &'a VolatileData)> {
        let mut vec_container = Vec::new();
        let mut vec_data = Vec::new();

        // Early return for when data contains nothing
        if data.is_empty() {
            return vec_container.into_iter().zip(vec_data);
        }

        // Map all data entries to specific containers
        if containers.len() == 1 {
            let container = containers.first();
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
                if let Some(name) = d.name.as_ref() {
                    if let Some(container) = mapped_all_containers.get(name) {
                        vec_container.push(container);
                        vec_data.push(d);
                    }
                    else {
                        error!("Couldn't find docker container {name} for volatile docker data");
                    }
                }
                else {
                    error!("Multiple docker containers found but volatile docker data does not have name to specific docker container. Cannot upload volatile docker data.")
                }
                
            };
        }

        vec_container.into_iter().zip(vec_data)
    }


    /// Upload all data specified in the data field of the [`Object`] to docker container
    pub fn upload_data(&self) -> Result<(), String> {
        for (container, data) in Self::iter_combine_data(&self.docker_manager.containers_ref_vec(), self.resources.get_docker_data()) {
            if let Some(ref ssh) = container.ssh {
                let path = Path::new(&data.source);
                if path.is_dir() {
                    ssh.upload_directory(path, data.destination.as_ref())?;
                }
                else if path.is_file() {
                    ssh.upload_file(path, data.destination.as_ref())?;
                }
                else {
                    panic!("Unknown data. Neither a valid file nor directory ({})", path.display());
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

        for (container, data) in Self::iter_combine_sftp_data(&self.docker_manager.containers_ref_vec(), self.resources.get_volatile_docker_data()) {
            if let Some(ref ssh) = container.ssh {
                let result = ssh.create_sftp_file(&data.destination);
                if let Ok(mut file) = result {
                    let result = file.write_all(data.content.as_bytes());
                    if let Err(error) = result {
                        error!("{error}");
                    }
                }
                else if let Err(error) = result {
                    error!("{error}");
                }
            } 
            else {
                panic!("Ssh client unavailable for container {} even though upload data was specified", container.config.name.as_ref().unwrap());
            }
        }

        Ok(())
    }
}
