use serde::{Deserialize, Serialize};

use crate::core::{object::Object, traits::Configurator};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "redis")]
pub struct Redis {}

impl Default for Redis {
    fn default() -> Redis {
        Redis {}
    }
}

impl Configurator<Object> for Redis {
    fn configure(&mut self, parent: &mut Object) {
        let container = parent.docker_container_builder.get_or_insert_default();

        container.try_name("redis").image("redis").ignore_ssh(true);
    }
}
