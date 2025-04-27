use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Storm {}


impl Storm {
    pub fn new() -> Self {
        Storm { }
    }

    pub fn setup_container(&self, docker: &mut Data) {
        docker.set_compose("lib/TrackBench.rs/compose-storm.yaml");
    }
}

