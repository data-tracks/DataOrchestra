use log::debug;
use serde::{Deserialize, Serialize};

use crate::core::attach::attach_types::AttachTypeConfig;
use crate::core::object::{Graph, Object};
use crate::core::types::ServiceConfig;
use crate::interface::data::ExtDataTypes;
use crate::interface::docker::ExtDocker;
use crate::interface::execute::ExtExecutables;
use crate::interface::node::ExtNode;
use crate::shared::Amount;
use crate::shared::traits::ToInternal;

/// External representation of the internal [`Object`] object
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExtObject {
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
    pub executables: Amount<ExtExecutables>,
    #[serde(rename = "service")]
    pub service_config: Option<ServiceConfig>,
}

impl Default for ExtObject {
    fn default() -> Self {
        ExtObject {
            name: None,
            graph: Graph::default(),
            docker: Some(ExtDocker::default()),
            node: None,
            ansible: Some("scripts/ansible/ansible-setup.yml".to_string()),
            attach_config: Amount::None,
            resources: Amount::Single(ExtDataTypes::Data(super::data::ExtData {
                location: super::location::Location::Container,
                name: None,
                source: "".to_string(),
                destination: "".to_string(),
                dependency: None,
            })),
            executables: Amount::None,
            service_config: None,
        }
    }
}

impl ToInternal<Object> for ExtObject {
    fn to_internal(self) -> Object {
        let mut object = Object::default();

        object.name = self.name.unwrap_or("object".to_string());

        object.graph = self.graph;

        if let Some(node) = self.node {
            object.node = Some(node.to_internal());
        }

        if let Some(ansible) = self.ansible {
            object.ansible = ansible;
        }

        // Set Container(s) builder
        if let Some(docker) = self.docker {
            if docker.compose.is_some() {
                object.docker_group_builder = Some(docker.to_internal());
            } else {
                object.docker_container_builder = Some(docker.to_internal());
            }
        }

        object.resources = self.resources.to_internal();
        let vec = self.executables.clone().to_internal();
        for (script, data) in vec {
            if let Some(data) = data {
                object.resources.push(data);
            }
            object.executables.push(script);
        }

        if let Some(service_config) = self.service_config {
            object.service_config = Some(service_config);
        }

        object
    }
}
