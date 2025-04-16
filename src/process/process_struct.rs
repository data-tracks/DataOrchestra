use serde::{Deserialize, Serialize};

use crate::{docker::docker_struct::Container, types::address::Address};

#[derive(Debug, Deserialize, Serialize)]
pub struct Process {
    #[serde(default = "default_amount")]
    pub amount: usize,
    pub docker: Option<Container>,
    pub remote: Option<Address>,
    pub script: String
}

pub fn default_amount() -> usize {
    1
}
