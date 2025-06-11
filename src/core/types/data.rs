use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub struct NodeData {
    pub path: String,
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

#[derive(Debug)]
pub struct DockerData {
    pub name: String,
    pub path: String,
    pub destination: String,
    pub env: Option<HashMap<String, String>>,
    pub start: String,
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

#[derive(Debug, Clone)]
pub struct DockerSFTPData {
    pub name: String,
    pub file: PathBuf,
    pub data: String
}

impl DockerSFTPData {
    pub fn new(name: impl Into<String>, file: PathBuf, data: impl Into<String>) -> Self {
        DockerSFTPData 
        {
            name: name.into(),
            data: data.into(),
            file
        }
    }
}
