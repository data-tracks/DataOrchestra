use serde::{Deserialize, Serialize};

use crate::{docker::docker_struct::Container, types::amount::Amount};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "polypheny")]
pub struct Polypheny {}

impl Polypheny {
    pub fn new() -> Polypheny {
        Polypheny {  }
    }

    pub fn setup_container(&self, docker: Container) -> Container {
        docker
    }

    pub fn mount_data(&self, schema: Amount<String>, mut docker: Container) -> Container {
        docker
    }
}
