use serde::{Deserialize, Serialize};

use crate::core::adapters::docker::container::ContainerBuilder;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "redis")]
pub struct Redis {
}

impl Redis {
    pub fn new() -> Redis {
        Redis {  }
    }

    pub fn setup_container(&self, docker: &mut ContainerBuilder) {
        docker
            .try_name_mut("redis")
            .image_mut("redis");
    }
}
