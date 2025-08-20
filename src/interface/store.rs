use std::path::Path;

use log::debug;
use serde::{Deserialize, Serialize};
use crate::core::object::Object;
use crate::core::store::store_types::{StoreType, StoreTypeConfig};
use crate::core::store::Store;
use crate::core::types::data::{DataBuilder, DataTypes};
use crate::interface::object::ExtObject;
use crate::shared::traits::ToInternal;
use crate::shared::Amount;

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
    pub object: ExtObject
}

impl Default for ExtStore {
    fn default() -> Self {
        ExtStore 
        {
            db_type: None,
            config: None,
            schema: Amount::None,
            object: ExtObject::default()
        }
    }
}

impl ToInternal<Store> for ExtStore {
    fn to_internal(self) -> Store {
        let mut store = Store::default();

        store.object = self.object.to_internal();

        if store.object.name.eq(&Object::default_name()) {
            store.object.name = "store".to_string();
        }

        // Set Database Type and config
        store.db_type = self.db_type;
        store.config = self.config;

        // Set Schema(s)
        store.schema = self.schema.to_vec(); 
        // Schema needs to be uploaded to the node for it to be mounted
        if store.object.node.is_some() {
            for schema in store.schema.iter_mut() {
                if let Some(file_name) = Path::new(schema)
                        .file_name()
                        .and_then(|name| name.to_str()) 
                {
                    // Alter path to that of the remote location
                    let data = DataBuilder::default()
                        .src(schema.clone())
                        .dst(format!("docker/mount/{file_name}"))
                        .build()
                        .expect("Unable to build data for store schema");
                    store.object.resources.push(DataTypes::NodeData(data)); 

                    *schema = format!("docker/mount/{file_name}");
                } 
            }
        }

        debug!("Finished parsing store to internal");
        store
    }
}
