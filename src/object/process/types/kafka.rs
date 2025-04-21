use serde::{Deserialize, Serialize};

use crate::docker::docker_struct::Container;

#[derive(Debug, Deserialize, Serialize)]
pub struct Kafka {}


impl Kafka {
    pub fn new() -> Self {
        Kafka { }
    }

    pub fn setup_container(&self, docker: Container) -> Container {
        docker.set_compose("lib/TrackBench.rs/compose-kafka.yaml")
    }
}

