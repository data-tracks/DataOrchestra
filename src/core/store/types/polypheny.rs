use serde::{Deserialize, Serialize};

use crate::{docker::docker_struct::Data, types::amount::Amount};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "polypheny")]
pub struct Polypheny {}

impl Polypheny {
    pub fn new() -> Polypheny {
        Polypheny {  }
    }

    pub fn setup_container(&self, docker: &mut Data) {
    }

    pub fn mount_data(&self, schema: Amount<String>, docker: &mut Data) {
    }
}
