use crate::attach::attach_types::AttachTypeConfig;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use data_orchestra_engine::object::{Graph, Object};
use crate::amount::Amount;
use crate::traits::ToInternal;
use crate::types::data::ExtDataTypes;
use crate::types::docker::ExtDocker;
use crate::types::execute::ExtExecutables;
use crate::types::node::ExtNode;

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
    /// Attachable configuration
    #[serde(default)]
    #[serde(rename = "attach")]
    pub attach_config: Amount<AttachTypeConfig>,
    #[serde(default)]
    pub resources: Amount<ExtDataTypes>,
    #[serde(default)]
    pub executables: Amount<ExtExecutables>,
}

impl Default for ExtObject {
    fn default() -> Self {
        ExtObject {
            name: None,
            graph: Graph::default(),
            docker: Some(ExtDocker::default()),
            node: None,
            attach_config: Amount::None,
            resources: Amount::Single(ExtDataTypes::Data(super::data::ExtData {
                location: super::location::Location::Container,
                name: None,
                source: "".to_string(),
                destination: "".to_string(),
                dependency: None,
            })),
            executables: Amount::None,
        }
    }
}

impl ToInternal<Object> for ExtObject {
    fn to_internal(self) -> Object {
        let mut object = Object::default();

        object.graph = self.graph;

        if let Some((node, uploader)) = self.node.to_internal() {
            object.node = Some(node);
            object.uploader = Some(uploader);
        }

        // Set Container(s) builder
        if let Some(docker) = self.docker {
            if docker.compose.is_some() {
                object.docker_compose_builder = Some(docker.to_internal());
            } else {
                object.docker_container_builder = Some(docker.to_internal());
            }
        }

        let resources = self.resources.to_internal();
        for (data, mount) in resources {
            object.resources.push(data);

            if let Some(mount) = mount {
                if let Some(container) = object.docker_container_builder.as_mut() {
                    container.mount(mount);
                } else {
                    error!("Mount data provided but no container available")
                }
            }
        }

        let vec = self.executables.clone().to_internal();
        for (script, data) in vec {
            if let Some(data) = data {
                object.resources.push(data);
            }
            object.executables.push(script);
        }

        debug!("Finished parsing object to internal");
        object
    }
}
