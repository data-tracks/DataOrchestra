use serde::{Deserialize, Serialize};

use crate::core::adapters::ComposeBuilder;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Flink {}

impl Flink {
    pub fn new() -> Self {
        Flink {}
    }

    pub fn setup_container(&self, docker: &mut ComposeBuilder) {
        docker.file("images/compose-flink.yaml");
    }
}
