use serde::{Deserialize, Serialize};

use crate::core::attach::attach_types::AttachTypeConfig;
use crate::core::object::Graph;
use crate::shared::{Amount, ExtDockerData, ExtNodeData};

use super::{docker::ExtDocker, node::ExtNode};

/// The general object. Represents general attributes of external representation objects
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct General {
    /// Name of object
    pub name: Option<String>,
    #[serde(default)]
    /// Graph structure of object
    #[serde(flatten)]
    pub graph: Graph,
    /// Docker container
    pub docker: Option<ExtDocker>, 
    /// Remote node connection
    pub node: Option<ExtNode>,
    #[serde(default)]
    pub node_data: Amount<ExtNodeData>,
    #[serde(default)]
    pub docker_data: Amount<ExtDockerData>,
    /// Ansible configuration script
    pub ansible: Option<String>,
    /// Attachable configuration
    #[serde(default)]
    #[serde(rename = "attach")]
    pub attach_config: Amount<AttachTypeConfig>,
}

impl Default for General {
    fn default() -> Self {
        General 
        { 
            name: None,
            graph: Graph::default(),
            docker: Some(ExtDocker::default()), 
            node: Some(ExtNode::default()), 
            node_data: Amount::Single(ExtNodeData::default()), 
            docker_data: Amount::Single(ExtDockerData::default()), 
            ansible: Some("".to_string()), 
            attach_config: Amount::Single(AttachTypeConfig::default()),
        }
    }  
}

