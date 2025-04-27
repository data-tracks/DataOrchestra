use serde::{Deserialize, Serialize};

use crate::core::adapters::docker::{Container, ContainerType, MultiContainer};
use crate::core::store::store_types::{StoreType, StoreTypeConfig};
use crate::core::store::Store;
use crate::shared::traits::ToInternal;
use crate::shared::Amount;

use super::config::General;

#[derive(Debug, Deserialize, Serialize)]
pub struct ExtStore {
    #[serde(rename = "type")]
    pub db_type: Option<StoreType>,
    pub config: Option<StoreTypeConfig>,
    pub schema: Amount<String>,
    #[serde(flatten)]
    pub general: General
}

impl ToInternal<Amount<Store>> for Amount<ExtStore> {
    fn to_internal(self) -> Amount<Store> {
        match self {
            Amount::None => Amount::None,
            Amount::Single(store) => Amount::Single(store.to_internal()),
            Amount::Multiple(stores) => {
                let mut vec_stores = Vec::<Store>::new();
                for store in stores {
                    vec_stores.push(store.to_internal());
                };

                Amount::Multiple(vec_stores)
            }
        }
    }
}

impl ToInternal<Store> for ExtStore {
    fn to_internal(self) -> Store {
        let mut store = Store::default();

        // Set Schema(s)
        store.schema = 
            match self.schema {
                Amount::None => Vec::new(),
                Amount::Single(schema) => vec![schema],
                Amount::Multiple(schemas) => schemas
            };
    
        // Set Database Type and config
        store.db_type = self.db_type;
        store.config = self.config;

        store.object.node = self.general.node;
        if let Some(docker) = self.general.docker {
            if let Some(compose) = docker.compose {
                store.object.docker = Some(ContainerType::Multiple(
                    MultiContainer::new(compose, Vec::new())
                ));
            }
            else 
            {
                store.object.docker = Some(ContainerType::Single(
                    Container::new(
                        docker.image,
                        docker.dockerfile,
                        docker.build_args,
                        docker.name,
                        docker.network,
                        docker.options,
                        Some(docker.mount.to_vec()),
                        docker.publish_all
                        )
                ));
            } 
        }

        store
    }
}
