use serde::{Deserialize, Serialize};

use crate::docker::docker_struct::Container;

#[derive(Debug, Deserialize, Serialize)]
pub struct Spark {}


impl Spark {
    pub fn new() -> Self {
        Spark { }
    }

    pub fn setup_container(&self, docker: &mut Container) {
        docker.set_compose("lib/TrackBench.rs/compose-spark.yaml");
    }
}

