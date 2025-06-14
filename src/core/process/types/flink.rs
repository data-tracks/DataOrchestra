use serde::{Deserialize, Serialize};

use crate::core::adapters::docker::ComposeGroupBuilder;

#[derive(Debug, Serialize, Deserialize)]
pub struct Flink {}

impl Flink {
    pub fn new() -> Self {
        Flink { }
    }

    pub fn setup_container(&self, docker: &mut ComposeGroupBuilder) {
        docker.compose_mut("images/compose-flink.yaml");
    }
}




