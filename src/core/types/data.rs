use std::path::PathBuf;

use derive_builder::Builder;

#[derive(Debug, Clone)]
pub enum DataTypes {
    VolatileNodeData(VolatileData),
    VolatileDockerData(VolatileData),
    DockerData(Data),
    NodeData(Data)
}

impl DataTypes {
    pub fn get_volatile_node_data_ref(&self) -> &VolatileData {
        match self {
            DataTypes::VolatileNodeData(data) => data,
            _ => panic!("Get volatile node on non-volatile node")
        }
    }

    pub fn get_volatile_docker_data_ref(&self) -> &VolatileData {
        match self {
            DataTypes::VolatileDockerData(data) => data,
            _ => panic!("Get volatile node on non-volatile docker")
        }
    }

    pub fn get_node_data_ref(&self) -> &Data {
        match self {
            DataTypes::NodeData(data) => data,
            _ => panic!("Get data node on non-data node")
        }
    }

    pub fn get_docker_data_ref(&self) -> &Data {
        match self {
            DataTypes::DockerData(data) => data,
            _ => panic!("Get volatile node on non-volatile docker")
        }
    }

    pub fn get_volatile_node_data_mut(&mut self) -> &mut VolatileData {
        match self {
            DataTypes::VolatileNodeData(data) => data,
            _ => panic!("Get volatile node on non-volatile node")
        }
    }

    pub fn get_volatile_docker_data_mut(&mut self) -> &mut VolatileData {
        match self {
            DataTypes::VolatileDockerData(data) => data,
            _ => panic!("Get volatile node on non-volatile docker")
        }
    }

    pub fn get_node_data_mut(&mut self) -> &mut Data {
        match self {
            DataTypes::DockerData(data) => data,
            _ => panic!("Get volatile node on non-volatile node")
        }
    }

    pub fn get_docker_data_mut(&mut self) -> &mut Data {
        match self {
            DataTypes::NodeData(data) => data,
            _ => panic!("Get volatile node on non-volatile docker")
        }
    }
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
    pub destination: PathBuf,
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
    pub source: String,
    /// Path of data in docker container
    #[builder(setter(into))]
    pub destination: String,
    #[builder(setter(strip_option, into))]
    #[builder(default)]
    pub dependency: Option<String>,
}