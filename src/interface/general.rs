use serde::{Deserialize, Serialize};

use crate::core::{attach::attach_types::AttachTypeConfig};
use crate::core::object::Graph;
use crate::interface::data::ExtDataTypes;
use crate::shared::Amount;

use super::execute::ExtExecutables;
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
    /// Ansible configuration script
    pub ansible: Option<String>,
    /// Attachable configuration
    #[serde(default)]
    #[serde(rename = "attach")]
    pub attach_config: Amount<AttachTypeConfig>,
    #[serde(default)]
    pub resources: Amount<ExtDataTypes>,
    #[serde(default)]
    pub executables: Amount<ExtExecutables>
}

impl Default for General {
    fn default() -> Self {
        General 
        { 
            name: None,
            graph: Graph::default(),
            docker: Some(ExtDocker::default()), 
            node: None, 
            ansible: Some("scripts/ansible/ansible-setup.yml".to_string()),
            attach_config: Amount::None,
            resources: Amount::Single(ExtDataTypes::Data(super::data::ExtData { location: super::location::Location::Container, name: None, source: "".to_string(), destination: "".to_string(), dependency: None })),
            executables: Amount::None
        }
    }  
}
