use crate::core::adapters::{
    ComposeBuilder, Container, ContainerBuilder, ContainerType, Executor, Local, Run, Uploader,
};
use crate::core::traits::Spawner;
use crate::core::types::data::{Data, DataTypes, GetVecData, VolatileData};
use crate::core::types::{Executables, GetExecutables, Node, Script};
use derive_builder::Builder;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Represents connection in the distributed system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    /// Where data from the object goes to
    #[serde(default)]
    pub to: Vec<String>,
    /// If object should be ignored when parsing to the graph structure
    #[serde(default)]
    pub ignore: bool,
}

impl Default for Graph {
    fn default() -> Self {
        Graph {
            to: Vec::new(),
            ignore: false,
        }
    }
}

/// The `Object` type. Acts as a generic component. Implements basic fields that every object
/// should possess.
#[derive(Debug, Builder)]
#[builder(pattern = "owned")]
pub struct Object {
    // Name of object. Default is the object type itself
    #[builder(setter(into), default = "Object::default_name()")]
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
    #[builder(default = "Object::default_executor()")]
    pub executor: Box<dyn Executor + Send + Sync>,
    #[builder(default)]
    pub uploader: Option<Box<dyn Uploader + Send + Sync>>,
}
impl ObjectBuilder {
    pub fn ignore_graph(mut self, ignore_graph: bool) -> Self {
        let graph = self.graph.get_or_insert_default();
        graph.ignore = ignore_graph;
        self
    }

    pub fn data(self, data: Data) -> Self {
        self.resource(DataTypes::Data(data))
    }

    pub fn volatile_data(self, volatile: VolatileData) -> Self {
        self.resource(DataTypes::VolatileData(volatile))
    }

    pub fn script(self, script: Script) -> Self {
        self.executable(Executables::Script(script))
    }
}

impl Default for Object {
    fn default() -> Self {
        Object {
            name: Object::default_name(),
            graph: Graph::default(),
            docker_group_builder: None,
            docker_container_builder: None,
            docker_manager: ContainerType::Empty,
            node: None,
            resources: Vec::new(),
            executables: Vec::new(),
            executor: Object::default_executor(),
            uploader: None,
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
            let mut compose = group.build().expect("Unable to build compose");
            // If a node was specified, docker compose file needs to be uploaded to node
            if self.node.is_some() {
                if let Some(uploader) = self.uploader.as_ref() {
                    compose.executor = self.executor.clone_box();

                    // Create remote directory for docker related files
                    let result = self.executor.exec("mkdir docker/".to_string());
                    if let Err(error) = result {
                        error!("Unable to create docker/ folder | {error}");
                    }
                    // Get filename of the docker compose file
                    let local_path = compose.config.compose.clone().unwrap();
                    if let Some(file_name) = Path::new(compose.config.compose.as_ref().unwrap())
                        .file_name()
                        .and_then(|name| name.to_str())
                    {
                        compose.config.compose = Some(format!("docker/{file_name}"));
                    }
                    // Upload user docker compose file
                    let result = uploader.upload_file(
                        local_path.as_ref(),
                        compose.config.compose.as_ref().unwrap().as_ref(),
                    );
                    if let Err(error) = result {
                        panic!("Unable to upload compose file {error}");
                    }
                } else {
                    panic!(
                        "No executor available for docker compose. Node was provided but ssh connection was not properly loaded"
                    );
                }
            }
            self.docker_manager = ContainerType::Compose(compose);
        }
        // Take ownership of ContainerBuilder out of object to prevent partial move
        else if let Some(container) = self.docker_container_builder.take() {
            info!("Setting up docker container");
            let mut container = container.build().expect("Unable to build container");

            // If a node was specified, dockerfile needs to be uploaded to node
            if self.node.is_some()
                && let Some(uploader) = self.uploader.as_ref()
            {
                container.executor = self.executor.clone_box();

                if let Some(dockerfile) = container.config.dockerfile.as_mut() {
                    let result = self.executor.exec("mkdir docker/".to_string());
                    if let Err(error) = result {
                        error!("Unable to create docker/ folder on node ({error})");
                    }
                    // Get filename of the dockerfile
                    let local_path = dockerfile.clone();
                    if let Some(file_name) = Path::new(dockerfile)
                        .file_name()
                        .and_then(|name| name.to_str())
                    {
                        *dockerfile = format!("docker/{file_name}");
                    }
                    // Upload dockerfile
                    let result = uploader.upload_file(local_path.as_ref(), dockerfile.as_ref());
                    if let Err(error) = result {
                        panic!("Unable to upload dockerfile ({error})");
                    }
                }
            }
            self.docker_manager = ContainerType::Container(container);
        } else {
            panic!("No Docker Builder available");
        }

        // Upload all node data to relevant node. This is done before the setup as the
        // specialization setup may start before object setup, thus data could be missing
        debug!("Uploading node data");
        if self.node.is_some() {
            if let Some(uploader) = self.uploader.as_ref() {
                for data in self.resources.get_data_ref() {
                    let path = Path::new(&data.src);
                    if path.is_dir() {
                        let result = uploader.upload_directory(path, data.dst.as_ref());
                        if let Err(error) = result {
                            error!("{error}");
                        }
                    } else if path.is_file() {
                        let result = uploader.upload_file(path, data.dst.as_ref());
                        if let Err(error) = result {
                            error!("{error}");
                        }
                    }
                }
            } else {
                error!("Node was provided but no uploader available");
            }
        }

        info!("Finished building {}", self.name);
    }

    /// Setup of object
    /// - Docker container functionally running
    /// - Docker container configured with base libraries
    /// - Docker container data uploaded
    fn setup(&mut self) {
        info!("Setting up {}", self.name);

        let result = self.docker_manager.run();
        if let Err(error) = result {
            error!("{error}");
        }

        info!("Finished setting up {}", self.name);
    }

    /// Deployment of Object. After invocation:
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
    pub fn default_name() -> String {
        "object".to_string()
    }

    pub fn default_executor() -> Box<dyn Executor + Send + Sync> {
        Box::new(Local::new())
    }

    /// Start starting script on remote object
    pub fn start_script(&self) -> Result<(), String> {
        for (container, script) in Self::iter_combine_script(
            &self.docker_manager.containers_ref_vec(),
            self.executables.get_scripts(),
        ) {
            info!(
                "Starting {} for {}",
                script.path,
                container.config.name.as_ref().unwrap()
            );

            let result = self.executor.exec(format!(
                "test -f {} && echo \"ok\" || echo \"err\"",
                script.path
            ));
            if let Err(error) = result {
                error!("{error}");
            } else if let Ok(result) = result {
                if result.eq("err") {
                    error!(
                        "Unable to find file {}. Check if the path is correctly formatted",
                        script.path
                    );
                }
            }

            if script.path.ends_with(".sh") {
                self.executor.exec(format!("sh {}", script.path))?;
            } else {
                self.executor.exec(script.path.to_string())?;
            }
        }

        Ok(())
    }

    /// Combine containers with [`Script`] to [`Iterator`]
    ///
    /// If `script` is empty, returns an empty iterator.
    ///
    /// If there is only on container, all entries in `script` get combined with that
    /// specific `container`
    pub fn iter_combine_script<'a>(
        containers: &'a Vec<&'a Container>,
        script: Vec<&'a Script>,
    ) -> impl Iterator<Item = (&'a Container, &'a Script)> {
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
        else {
            let mut mapped_all_containers = HashMap::<&String, &Container>::new();
            for container in containers.iter() {
                mapped_all_containers.insert(container.config.name.as_ref().unwrap(), container);
            }

            for s in script.iter() {
                if let Some(name) = s.name.as_ref() {
                    if let Some(container) = mapped_all_containers.get(&name) {
                        vec_container.push(container);
                        vec_script.push(s);
                    } else {
                        error!("Couldn't find docker container {name} for docker data");
                    }
                } else {
                    error!(
                        "Multiple docker containers found but docker data does not have name to specific docker container. Cannot upload docker data."
                    )
                }
            }
        }

        vec_container.into_iter().zip(vec_script)
    }

    /// Combine containers with [`DockerData`] to [`Iterator`]
    ///
    /// If `data` is empty, returns an empty iterator.
    ///
    /// If there is only on container, all entries in `data` get combined with that
    /// specific `container`
    pub fn iter_combine_data<'a>(
        containers: &'a Vec<&'a Container>,
        data: Vec<&'a Data>,
    ) -> impl Iterator<Item = (&'a Container, &'a Data)> {
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
        else {
            let mut mapped_all_containers = HashMap::<&String, &Container>::new();
            for container in containers.iter() {
                mapped_all_containers.insert(container.config.name.as_ref().unwrap(), container);
            }

            for d in data.iter() {
                if let Some(name) = d.name.as_ref() {
                    if let Some(container) = mapped_all_containers.get(name) {
                        vec_container.push(container);
                        vec_data.push(d);
                    } else {
                        error!("Couldn't find docker container {name} for docker data");
                    }
                } else {
                    error!(
                        "Multiple docker containers found but docker data does not have name to specific docker container. Cannot upload docker data."
                    )
                }
            }
        }

        vec_container.into_iter().zip(vec_data)
    }

    /// Combine containers with [`DockerSFTPData`] to [`Iterator`]
    ///
    /// If `data` is empty, returns an empty iterator.
    ///
    /// If there is only on container, all entries in `data` get combined with that
    /// specific `container`
    pub fn iter_combine_sftp_data<'a>(
        containers: &'a Vec<&'a Container>,
        data: Vec<&'a VolatileData>,
    ) -> impl Iterator<Item = (&'a Container, &'a VolatileData)> {
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
        else {
            let mut mapped_all_containers = HashMap::<&String, &Container>::new();
            for container in containers.iter() {
                mapped_all_containers.insert(container.config.name.as_ref().unwrap(), container);
            }

            for d in data.iter() {
                if let Some(name) = d.name.as_ref() {
                    if let Some(container) = mapped_all_containers.get(name) {
                        vec_container.push(container);
                        vec_data.push(d);
                    } else {
                        error!("Couldn't find docker container {name} for volatile docker data");
                    }
                } else {
                    error!(
                        "Multiple docker containers found but volatile docker data does not have name to specific docker container. Cannot upload volatile docker data."
                    )
                }
            }
        }

        vec_container.into_iter().zip(vec_data)
    }
}
