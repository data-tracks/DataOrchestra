use serde::{Deserialize, Serialize};

use crate::shared::{Amount, File};

use super::{docker::ExtDocker, node::ExtNode};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct General {
    pub docker: Option<ExtDocker>, 
    pub node: Option<ExtNode>,
    #[serde(default)]
    pub node_files: Amount<File>,
    #[serde(default)]
    pub docker_files: Amount<File>,
    pub ansible: Option<String>
}

