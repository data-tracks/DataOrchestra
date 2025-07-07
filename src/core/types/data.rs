use std::path::PathBuf;

use derive_builder::Builder;

#[derive(Debug, Clone)]
pub enum DataTypes {
    VolatileNodeData(VolatileData),
    VolatileDockerData(VolatileData),
    DockerData(Data),
    NodeData(Data)
}

pub trait GetData {
    fn get_volatile_docker_data(&self) -> Vec<&VolatileData>;
    fn get_volatile_node_data(&self) -> Vec<&VolatileData>;
    fn get_docker_data(&self) -> Vec<&Data>;
    fn get_node_data(&self) -> Vec<&Data>;
}

impl GetData for Vec<DataTypes> {
    fn get_volatile_docker_data(&self) -> Vec<&VolatileData> {
        let mut vec = Vec::new();
        for item in self {
            match item {
                DataTypes::VolatileDockerData(data) => vec.push(data),        
                _ => ()
            }
        }

        vec
    }

    fn get_volatile_node_data(&self) -> Vec<&VolatileData> {
        let mut vec = Vec::new();
        for item in self {
            match item {
                DataTypes::VolatileNodeData(data) => vec.push(data),        
                _ => ()
            }
        }

        vec
    }

    fn get_node_data(&self) -> Vec<&Data> {
        let mut vec = Vec::new();
        for item in self {
            match item {
                DataTypes::NodeData(data) => vec.push(data),        
                _ => ()
            }
        }

        vec
    }

    fn get_docker_data(&self) -> Vec<&Data> {
        let mut vec = Vec::new();
        for item in self {
            match item {
                DataTypes::DockerData(data) => vec.push(data),        
                _ => ()
            }
        }

        vec
    }
}


#[derive(Debug, Clone, Builder)]
pub struct VolatileData {
    /// Name of docker container
    #[builder(setter(strip_option, into), default)]
    pub name: Option<String>,
    /// File name
    #[builder(setter(into))]
    pub file: PathBuf,
    /// Data written into file
    #[builder(setter(into))]
    pub content: String
}

#[derive(Debug, Clone, Builder)]
pub struct Data {
    /// Name of container
    #[builder(setter(strip_option, into), default)]
    pub name: Option<String>,
    /// Path of data in current system
    #[builder(setter(into))]
    pub path: String,
    /// Path of data in docker container
    #[builder(setter(into))]
    pub destination: String,
    #[builder(setter(strip_option, into))]
    #[builder(default)]
    pub dependency: Option<String>,
}
/*

/// Node data object. Represents data that gets uploaded onto the node
#[derive(Debug, Clone, Builder)]
pub struct NodeData {
    /// Path of data in current system
    #[builder(setter(into))]
    pub path: String,
    /// Path of data in remote system
    #[builder(setter(into))]
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
#[derive(Debug, Builder, Clone)]
pub struct DockerData {
    /// Name of container
    #[builder(setter(into), default)]
    pub name: Option<String>,
    /// Path of data in current system
    #[builder(setter(into))]
    pub path: String,
    /// Path of data in docker container
    #[builder(setter(into))]
    pub destination: String,
    /// Environment variables
    #[builder(setter(each = "entry", into))]
    #[builder(default)]
    pub env: Option<HashMap<String, String>>,
    /// Starting script / command
    #[builder(setter(into))]
    pub start: String,
    /// Path of script which has dependencies which need to downloaded / handled
    #[builder(setter(strip_option, into))]
    #[builder(default)]
    pub dependency: Option<String>,
    #[builder(default)]
    pub tmux: Option<Tmux>
}

impl Default for DockerData {
    fn default() -> Self {
        DockerData 
        {
            name: None,
            path: String::new(),
            destination: "/".to_string(),
            env: Some(HashMap::new()),
            start: String::new(),
            dependency: None,
            tmux: None
        }
    }
}

impl DockerData {
    pub fn new(name: Option<impl Into<String>>, path: impl Into<String>, destination: impl Into<String>, start: impl Into<String>, env: Option<HashMap<String, String>>, dependency: Option<String>, tmux: Option<Tmux>) -> Self {
        DockerData 
        {
            name: name.map(|n| n.into()),
            path: path.into(),
            destination: destination.into(),
            env,
            start: start.into(),
            dependency,
            tmux
        }
    }
}

/// Docker volatile data object. Represents data which is written to file
/// in the docker container from memory
#[derive(Debug, Clone, Builder)]
pub struct VolatileDockerData {
    /// Name of docker container
    #[builder(default)]
    #[builder(setter(strip_option, into))]
    pub name: Option<String>,
    /// File name 
    #[builder(setter(into))]
    pub file: PathBuf,
    /// Data written into file
    #[builder(setter(into))]
    pub content: String
}

pub struct VolatileNodeData {

}

impl VolatileDockerData {
    pub fn new(name: Option<impl Into<String>>, file: PathBuf, data: impl Into<String>) -> Self {
        VolatileDockerData 
        {
            name: name.map(|n| n.into()),
            content: data.into(),
            file
        }
    }
}
*/

