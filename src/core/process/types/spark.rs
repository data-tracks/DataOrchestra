use serde::{Deserialize, Serialize};

use crate::core::adapters::docker::ComposeGroupBuilder;

#[derive(Debug, Serialize, Deserialize)]
pub struct Spark {}

impl Spark {
    pub fn new() -> Self {
        Spark { }
    }

    pub fn setup_container(&self, docker: &mut ComposeGroupBuilder) {
        docker.set_compose("images/compose-spark.yaml");
    }
}

