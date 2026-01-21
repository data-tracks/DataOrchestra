use serde::{Deserialize, Serialize};

use crate::adapters::ComposeBuilder;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Spark {}

impl Spark {
    pub fn new() -> Self {
        Spark {}
    }

    pub fn setup_container(&self, docker: &mut ComposeBuilder) {
        docker.compose("images/compose-spark.yaml");
    }
}
