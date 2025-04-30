use serde::{Deserialize, Serialize};

use crate::core::adapters::docker::ComposeGroupBuilder;

#[derive(Debug, Serialize, Deserialize)]
pub struct Storm {}

impl Storm {
    pub fn new() -> Self {
        Storm { }
    }

    pub fn setup_container(&self, docker: &mut ComposeGroupBuilder) {
        docker.set_compose("lib/TrackBench.rs/compose-storm.yaml");
    }
}

