use serde::{Deserialize, Serialize};

use crate::{docker::docker_struct::Container, types::amount::Amount};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "redis")]
pub struct Redis {
}

impl Redis {
    pub fn new() -> Redis {
        Redis {  }
    }

    pub fn setup_container(&self, docker: Container) -> Container {
        docker.set_image("redis")
    }

    pub fn mount_data(&self, schema: Amount<String>, mut docker: Container) -> Container {
        docker
    }
}
