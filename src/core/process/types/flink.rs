use serde::{Deserialize, Serialize};

use crate::docker::docker_struct::Data;

#[derive(Debug, Deserialize, Serialize)]
pub struct Flink {
}

impl Flink {
    pub fn new() -> Self {
        Flink { }
    }

    pub fn setup_container(&self, docker: &mut Data) {
        docker.set_compose("lib/TrackBench.rs/compose-flink.yaml");
    }
}




