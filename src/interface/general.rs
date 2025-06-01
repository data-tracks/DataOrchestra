use serde::{Deserialize, Serialize};

use crate::shared::{Amount, DockerFile, NodeFile};

use super::{docker::ExtDocker, node::ExtNode};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct General {
    pub docker: Option<ExtDocker>, 
    pub node: Option<ExtNode>,
    #[serde(default)]
    pub node_data: Amount<NodeFile>,
    #[serde(default)]
    pub docker_data: Amount<DockerFile>,
    pub ansible: Option<String>
}

