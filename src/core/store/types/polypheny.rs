use serde::{Deserialize, Serialize};

use crate::{core::adapters::docker::{container::ContainerBuilder, Container}, shared::Amount};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "polypheny")]
pub struct Polypheny {}

impl Polypheny {
    pub fn new() -> Polypheny {
        Polypheny {  }
    }

    pub fn setup_container(&self, docker: &mut ContainerBuilder) {
    }

    pub fn mount_data(&self, schema: Amount<String>, docker: &mut ContainerBuilder) {
    }
}
