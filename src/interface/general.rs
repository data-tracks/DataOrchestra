use serde::{Deserialize, Serialize};

use crate::shared::{Amount, File, ExtNode};

use super::docker::ExtDocker;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct General {
    pub docker: Option<ExtDocker>, 
    pub node: Option<ExtNode>,
    #[serde(default)]
    pub file: Amount<File>,
}

