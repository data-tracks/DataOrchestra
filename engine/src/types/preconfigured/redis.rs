use serde::{Deserialize, Serialize};

use crate::{ traits::Configurable};
use crate::object::Object;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename = "redis")]
pub struct Redis {}

impl Default for Redis {
    fn default() -> Redis {
        Redis {}
    }
}

impl Configurable<Object> for Redis {
    fn configure(&mut self, parent: &mut Object) {
        let container = parent
            .docker_container_builder
            .get_or_insert_default();

        container.try_name("redis").image("redis");
    }
}
