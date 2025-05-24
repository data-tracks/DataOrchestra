use log::debug;
use serde::{Deserialize, Serialize};
use crate::core::store::store_types::{StoreType, StoreTypeConfig};
use crate::core::store::Store;
use crate::shared::traits::ToInternal;
use crate::shared::Amount;

use super::general::General;

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

impl ToInternal<Store> for ExtStore {
    fn to_internal(self) -> Store {
        let mut store = Store::default();

        // Set Schema(s)
        store.schema = self.schema.to_vec(); 
    
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
            if docker.compose.is_some() || docker.names.is_some() {
                store.object.docker_group_builder = Some(docker.to_internal());
            }
            else 
            {
                store.object.docker_container_builder = Some(docker.to_internal());
            } 
        }
    
        store.object.data = self.general.file.to_internal();

        debug!("Finished parsing store to internal");
        store
    }
}
