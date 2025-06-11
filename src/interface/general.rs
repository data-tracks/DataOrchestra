use serde::{Deserialize, Serialize};

use crate::core::attach::attach_types::AttachTypeConfig;
use crate::core::object::Graph;
use crate::shared::{Amount, DockerFile, NodeFile};

use super::{docker::ExtDocker, node::ExtNode};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct General {
    pub name: Option<String>,
    #[serde(default)]
    #[serde(flatten)]
    pub graph: Graph,
    pub docker: Option<ExtDocker>, 
    pub node: Option<ExtNode>,
    #[serde(default)]
    pub node_data: Amount<NodeFile>,
    #[serde(default)]
    pub docker_data: Amount<DockerFile>,
    pub ansible: Option<String>,
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
            node_data: Amount::Single(NodeFile::default()), 
            docker_data: Amount::Single(DockerFile::default()), 
            ansible: Some("".to_string()), 
            attach_config: Amount::Single(AttachTypeConfig::default()),
        }
    }  
}

