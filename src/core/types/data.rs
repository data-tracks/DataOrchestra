use std::{collections::HashMap, path::PathBuf};

/// Node data object. Represents data that gets uploaded onto the node
#[derive(Debug)]
pub struct NodeData {
    /// Path of data in current system
    pub path: String,
    /// Path of data in remote system
    pub destination: String,
}

impl Default for NodeData {
    fn default() -> Self {
        NodeData { path: String::new(), destination: String::from("data/") }
    }
}

impl NodeData {
    pub fn new(path: impl Into<String>, destination: impl Into<String>) -> Self {
        NodeData { path: path.into(), destination: destination.into() }
    }
}

/// Docker data object. Represents data that gets uploaded into the docker container
#[derive(Debug)]
pub struct DockerData {
    /// Name of container
    pub name: String,
    /// Path of data in current system
    pub path: String,
    /// Path of data in docker container
    pub destination: String,
    /// Environment variables
    pub env: Option<HashMap<String, String>>,
    /// Starting script / command
    pub start: String,
    /// Path of script which has dependencies which need to downloaded / handled
    pub dependency: Option<String>
}

impl Default for DockerData {
    fn default() -> Self {
        DockerData 
        {
            name: String::new(),
            path: String::new(),
            destination: "/".to_string(),
            env: Some(HashMap::new()),
            start: String::new(),
            dependency: None
        }
    }
}

impl DockerData {
    pub fn new(name: impl Into<String>, path: impl Into<String>, destination: impl Into<String>, start: impl Into<String>, env: Option<HashMap<String, String>>, dependency: Option<String>) -> Self {
        DockerData 
        {
            name: name.into(),
            path: path.into(),
            destination: destination.into(),
            env,
            start: start.into(),
            dependency: dependency.map(|item| item.into())
        }
    }
}

/// Docker volatile data object. Represents data which is written to file
/// in the docker container from memory
#[derive(Debug, Clone)]
pub struct VolatileDockerData {
    /// Name of docker container
    pub name: String,
    /// File name 
    pub file: PathBuf,
    /// Data written into file
    pub data: String
}

impl VolatileDockerData {
    pub fn new(name: impl Into<String>, file: PathBuf, data: impl Into<String>) -> Self {
        VolatileDockerData 
        {
            name: name.into(),
            data: data.into(),
            file
        }
    }
}
