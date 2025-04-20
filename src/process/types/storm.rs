use serde::{Deserialize, Serialize};

use crate::docker::docker_struct::Container;

#[derive(Debug, Deserialize, Serialize)]
pub struct Storm {}


impl Storm {
    pub fn new() -> Self {
        Storm { }
    }

    pub fn setup_container(&self, docker: Container) -> Container {
        docker.set_compose("/lib/compose-storm.yaml")
    }
}

