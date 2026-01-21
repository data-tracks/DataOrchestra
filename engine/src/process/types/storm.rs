use serde::{Deserialize, Serialize};

use crate::adapters::ComposeBuilder;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Storm {}

impl Storm {
    pub fn new() -> Self {
        Storm { }
    }

    pub fn setup_container(&self, docker: &mut ComposeBuilder) {
        docker.compose("images/compose-storm.yaml");
    }
}
