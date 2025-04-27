use serde::{Deserialize, Serialize};

use crate::core::store::store_struct::Store;
use crate::core::store::store_types::{StoreType, StoreTypeConfig};
use crate::shared::traits::ToInternal;

use super::config::General;

#[derive(Debug, Deserialize, Serialize)]
pub struct ExtStore {
    #[serde(rename = "type")]
    db_type: Option<StoreType>,
    config: Option<StoreTypeConfig>,
    #[serde(flatten)]
    general: General
}

impl ToInternal<Store> for ExtStore {
    fn to_internal(self) -> Store {
        let mut store = Store::default();

        // Set type if database if specified. 
        store.db_type = self.db_type;
        if store.db_type.is_some() && self.config.is_some() {
            store.config = self.config
        }
        else if store.db_type.is_some() && self.config.is_none() {
            store.config = Some(store.db_type.as_ref().unwrap().new());
        }

        store
    }
}
