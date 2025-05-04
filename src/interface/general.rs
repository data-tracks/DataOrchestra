use serde::{Deserialize, Serialize};

use crate::shared::{Amount, File, Node};

use super::docker::ExtDocker;

#[derive(Debug, Deserialize, Serialize)]
pub struct General {
    pub docker: Option<ExtDocker>, 
    pub node: Option<Node>,
    #[serde(default)]
    pub file: Amount<File>,
}

