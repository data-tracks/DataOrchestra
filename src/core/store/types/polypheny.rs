use serde::{Deserialize, Serialize};

use crate::core::adapters::docker::container::ContainerBuilder;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "polypheny")]
pub struct Polypheny {}

impl Polypheny {
    pub fn new() -> Polypheny {
        Polypheny {  }
    }

    pub fn setup_container(&self, docker: &mut ContainerBuilder) {
    }
}
