use serde::{Deserialize, Serialize};

use crate::{store::Store, traits::Configurator};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "redis")]
pub struct Redis {}

impl Default for Redis {
    fn default() -> Redis {
        Redis {}
    }
}

impl Configurator<Store> for Redis {
    fn configure(&mut self, parent: &mut Store) {
        let container = parent
            .object
            .docker_container_builder
            .get_or_insert_default();

        container.try_name("redis").image("redis");
    }
}
