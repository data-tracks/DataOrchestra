use serde::{Deserialize, Serialize};

use crate::core::adapters::docker::Container;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "redis")]
pub struct Redis {
}

impl Redis {
    pub fn new() -> Redis {
        Redis {  }
    }

    pub fn setup_container(&self, docker: &mut Container) {
        docker.set_image("redis");
    }
}
