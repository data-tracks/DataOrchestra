use std::path::Path;

use log::debug;
use serde::{Deserialize, Serialize};
use crate::core::store::store_types::{StoreType, StoreTypeConfig};
use crate::core::store::Store;
use crate::core::types::data::NodeData;
use crate::shared::traits::ToInternal;
use crate::shared::Amount;

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
    pub general: General
}

impl Default for ExtStore {
    fn default() -> Self {
        ExtStore 
        {
            db_type: None,
            config: None,
            schema: Amount::None,
            general: General::default()
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
                if let Some(file_name) = Path::new(schema)
                        .file_name()
                        .and_then(|name| name.to_str()) 
                {
                    // Alter path to that of the remote location
                    store.object.node_data.push(NodeData::new(schema.clone().to_owned(), format!("docker/mount/{}", file_name))); 
                    *schema = format!("docker/mount/{}", file_name);
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

        // Set Container(s) builder
        if let Some(docker) = self.general.docker {
            if docker.compose.is_some() {
                store.object.docker_group_builder = Some(docker.to_internal());
            }
            else 
            {
                store.object.docker_container_builder = Some(docker.to_internal());
            } 
        }

        store.object.node_data.extend(self.general.node_data.to_internal());
        store.object.docker_datas.extend(self.general.docker_data.to_internal());

        debug!("Finished parsing store to internal");
        store
    }
}
