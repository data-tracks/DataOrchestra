use serde::{Deserialize, Serialize};

use crate::{docker::docker_struct::Container, types::amount::Amount};

#[derive(Debug, Deserialize, Serialize)]
pub struct Kafka {
    #[serde(default)]
    topic: Amount<String>
}


impl Kafka {
    pub fn new() -> Self {
        Kafka { topic: Amount::None }
    }

    pub fn setup_container(&self, docker: &mut Container) {
        docker.set_compose("lib/TrackBench.rs/compose-kafka.yaml");
    }
}

