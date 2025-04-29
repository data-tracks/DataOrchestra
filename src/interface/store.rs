use log::debug;
use serde::{Deserialize, Serialize};
use crate::core::adapters::docker::container::ContainerBuilder;
use crate::core::adapters::docker::ComposeGroupBuilder;
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
    #[serde(default)]
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

        // Set Container(s) builder
        store.object.node = self.general.node;
        if let Some(docker) = self.general.docker {
            if let Some(compose) = docker.compose {
                let mut builder = ComposeGroupBuilder::new();
                builder.set_compose(compose);
                store.object.docker_group_builder = Some(builder);
            }
            else 
            {
                let mut builder = ContainerBuilder::new();
                if let Some(name) = docker.name {
                    builder.set_name(name);
                }
                if let Some(image) = docker.image {
                    builder.set_image(image);
                }
                if let Some(dockerfile) = docker.dockerfile {
                    builder.set_dockerfile(dockerfile);
                }
                if let Some(build_args) = docker.build_args {
                    for (key, value) in build_args {
                        builder.add_build_arg(key, value);
                    }
                }
                if let Some(network) = docker.network {
                    builder.set_network(network);
                }
                if let Some(env) = docker.enviroment {
                    for (key, value) in env {
                        builder.add_env_var(key, value);
                    }
                }
                match docker.mount {
                    Amount::Single(mount) => {
                        builder.add_mount(mount);
                    }
                    ,
                    Amount::Multiple(mounts) => {
                        for mount in mounts {
                            builder.add_mount(mount);
                        }
                    },
                    Amount::None => ()
                }

                builder.set_publish_all(docker.publish_all);

                store.object.docker_container_builder = Some(builder);
            } 
        }

        debug!("Finished parsing to internal");
        dbg!("{}", &store);
        store
    }
}
