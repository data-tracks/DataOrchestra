use log::error;
use serde::{Deserialize, Serialize};
use crate::core::types::data::NodeData;
use crate::core::types::DockerData;
use crate::shared::traits::ToInternal;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NodeFile {
    pub path: String,
    pub destination: Option<String>
}

impl ToInternal<NodeData> for NodeFile {
    fn to_internal(self) -> NodeData {
        let mut data = NodeData::default();

        data.path = self.path;

        if let Some(destination) = self.destination {
            data.destination = destination;
        }

        data
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DockerFile {
    pub name: Option<String>,
    pub path: String,
    pub destination: Option<String>,
    pub start: Option<String>,
    pub dependency: Option<String>
}

impl ToInternal<DockerData> for DockerFile {
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
