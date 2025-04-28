use serde::{Deserialize, Serialize};

use crate::core::adapters::docker::{container::ContainerBuilder, Container};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "redis")]
pub struct Redis {
}

impl Redis {
    pub fn new() -> Redis {
        Redis {  }
    }

    pub fn setup_container(&self, docker: &mut ContainerBuilder) {
        docker.set_image("redis");
    }
}
