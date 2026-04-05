use std::path::Path;
use crate::attach::attach_types::AttachTypeConfig;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use data_orchestra_engine::object::{Graph, Object};
use data_orchestra_engine::types::{DataBuilder, DataTypes};
use crate::amount::Amount;
use crate::traits::ToInternal;
use crate::types::data::ExtDataTypes;
use crate::types::docker::ExtDocker;
use crate::types::execute::ExtExecutables;
use crate::types::node::ExtNode;
use data_orchestra_engine::types::preconfigured_type::{PreconfiguredType, PreconfiguredTypeConfig};

/// External representation of the internal [`Object`] object
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExtObject {
    #[serde(default = "ExtObject::default_amount")]
    pub amount: usize,
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
    pub preconfigured_type: Option<PreconfiguredType>,
    pub preconfigured_type_config: Option<PreconfiguredTypeConfig>
}

impl ExtObject {
    pub fn default_amount() -> usize {
        1
    }
}

impl Default for ExtObject {
    fn default() -> Self {
        ExtObject {
            name: None,
            amount: ExtObject::default_amount(),
            graph: Graph::default(),
            docker: Some(ExtDocker::default()),
            node: None,
            attach_config: Amount::None,
            resources: Amount::None,
            executables: Amount::None,
            preconfigured_type: None,
            preconfigured_type_config: None
        }
    }
}

impl ToInternal<Object> for ExtObject {
    fn to_internal(self) -> Object {
        let mut object = Object::default();

        object.preconfigured_type = self.preconfigured_type;
        object.preconfigured_type_config = self.preconfigured_type_config;

        if let Some(config) = object.preconfigured_type_config.as_mut()
            && let PreconfiguredTypeConfig::PostgreSQL(postgres) = config
            && self.node.is_some()
        {
            for schema in postgres.schema.iter_mut() {
                if let Some(file_name) =
                    Path::new(schema).file_name().and_then(|name| name.to_str())
                {
                    // Alter path to that of the remote location
                    let data = DataBuilder::default()
                        .src(schema.clone())
                        .dst(format!("docker/mount/{file_name}"))
                        .build()
                        .expect("Unable to build data for store schema");
                    object.resources.push(DataTypes::Data(data));

                    *schema = format!("docker/mount/{file_name}");
                }
            }
        }

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
