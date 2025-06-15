use std::collections::HashMap;
use std::path::PathBuf;

use log::error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::core::types::data::{NodeData, VolatileDockerData};
use crate::core::types::DockerData;
use crate::shared::traits::ToInternal;

/// External representation of the internal [`NodeData`] object 
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExtNodeData {
    pub path: String,
    pub destination: Option<String>
}

impl Default for ExtNodeData {
    fn default() -> Self {
        ExtNodeData 
        { 
            path: "".to_string(), 
            destination: Some("".to_string())
        }
    }
}

impl ToInternal<NodeData> for ExtNodeData {
    fn to_internal(self) -> NodeData {
        let mut data = NodeData::default();

        data.path = self.path;

        if let Some(destination) = self.destination {
            data.destination = destination;
        }

        data
    }
}

/// External representation of the internal [`DockerData`] object
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExtDockerData {
    pub name: Option<String>,
    pub path: String,
    pub destination: Option<String>,
    pub env: Option<HashMap<String, String>>,
    pub start: Option<String>,
    pub dependency: Option<String>
}

impl Default for ExtDockerData {
    fn default() -> Self {
        ExtDockerData 
        { 
            name: None, 
            path: "".to_string(), 
            destination: None, 
            env: None,
            start: None, 
            dependency: None
        }
    }
}

impl ToInternal<DockerData> for ExtDockerData {
    fn to_internal(self) -> DockerData {
        let mut data = DockerData::default();

        if let Some(name) = self.name {
            data.name = name;
        }
        
        data.path = self.path;

        if let Some(destination) = self.destination {
            if !destination.starts_with("/") {
                error!("File path doesn't start with /<path>");
            }
            data.destination = destination;
        }
        if let Some(start) = self.start {
            data.start = start;
        }

        data.dependency = self.dependency;

        data
    }
}

/// External representation of the internal [`VolatileDockerData`] object
#[derive(Debug, Deserialize, Serialize)]
pub struct ExtVolatileDockerData {
    pub name: String,
    pub file: PathBuf,
    pub data: Value
}

impl Default for ExtVolatileDockerData {
    fn default() -> Self {
        ExtVolatileDockerData 
        { 
            name: "".to_string(), 
            file: PathBuf::default(), 
            data: Value::Null 
        }
    }
}

impl ToInternal<VolatileDockerData> for ExtVolatileDockerData {
    fn to_internal(self) -> VolatileDockerData {
        let data = serde_json::to_string_pretty(&self.data).expect("Unable to parse data to string");
        VolatileDockerData::new(self.name, self.file, data) 
    }
}
