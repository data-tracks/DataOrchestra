use std::path::Path;

use crate::core::store::Store;
use crate::core::store::store_types::{StoreType, StoreTypeConfig};
use crate::core::types::data::{DataBuilder, DataTypes};
use crate::interface::docker::ExtDocker;
use crate::shared::Amount;
use crate::shared::traits::ToInternal;
use log::debug;
use serde::{Deserialize, Serialize};

use super::general::General;

/// External representation of the internal [`Store`] object
#[derive(Debug, Deserialize, Serialize)]
pub struct ExtStore {
    #[serde(rename = "type")]
    pub db_type: Option<StoreType>,
    #[serde(flatten)]
    pub config: Option<StoreTypeConfig>,
    #[serde(default)]
    pub schema: Amount<String>,
    #[serde(flatten)]
    pub general: General,
}

impl Default for ExtStore {
    fn default() -> Self {
        ExtStore {
            db_type: None,
            config: None,
            schema: Amount::None,
            general: General::default(),
        }
    }
}

impl ToInternal<Store> for ExtStore {
    fn to_internal(self) -> Store {
        let mut store = Store::default();

        store.object.name = self.general.name.unwrap_or("store".to_string());

        store.object.graph = self.general.graph;

        // Set Schema(s)
        store.schema = self.schema.to_vec();
        // Schema needs to be uploaded to the node for it to be mounted
        if self.general.node.is_some() {
            for schema in store.schema.iter_mut() {
                if let Some(file_name) =
                    Path::new(schema).file_name().and_then(|name| name.to_str())
                {
                    // Alter path to that of the remote location
                    let data = DataBuilder::default()
                        .source(schema.clone())
                        .destination(format!("docker/mount/{file_name}"))
                        .build()
                        .expect("Unable to build data for store schema");
                    store.object.resources.push(DataTypes::NodeData(data));

                    *schema = format!("docker/mount/{file_name}");
                }
            }
        }

        // Set Database Type and config
        store.db_type = self.db_type;
        store.config = self.config;

        if let Some(node) = self.general.node {
            store.object.node = Some(node.to_internal());
        }

        if let Some(ansible) = self.general.ansible {
            store.object.ansible = ansible;
        }

        if let Some(docker) = self.general.docker.clone() {
            match docker {
                ExtDocker::Compose(compose) => {
                    store.object.docker_group_builder = Some(compose.to_internal());
                }
                ExtDocker::Container(container) => {
                    store.object.docker_container_builder = Some(container.to_internal());
                }
                ExtDocker::Dind(dind) => {
                    store.object.docker_container_builder = Some(dind.to_internal());
                }
            };
        }

        store
            .object
            .resources
            .extend(self.general.resources.to_internal());
        let vec = self.general.executables.clone().to_internal();
        for (script, data) in vec {
            if let Some(data) = data {
                store.object.resources.push(data);
            }
            store.object.executables.push(script);
        }

        debug!("Finished parsing store to internal");
        store
    }
}
