use serde::{Deserialize, Serialize};

use crate::core::adapters::ComposeBuilder;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Spark {}

impl Spark {
    pub fn new() -> Self {
        Spark {}
    }

    pub fn setup_container(&self, docker: &mut ComposeBuilder) {
        docker.file("images/compose-spark.yaml");
    }
}
