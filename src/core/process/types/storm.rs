use serde::{Deserialize, Serialize};

use crate::core::adapters::ComposeBuilder;


#[derive(Debug, Serialize, Deserialize)]
pub struct Storm {}

impl Storm {
    pub fn new() -> Self {
        Storm { }
    }

    pub fn setup_container(&self, docker: &mut ComposeBuilder) {
        docker.compose("images/compose-storm.yaml");
    }
}

