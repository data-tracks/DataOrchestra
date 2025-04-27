use crate::core::adapters::docker::MultiContainer;

#[derive(Debug)]
pub struct Storm {}

impl Storm {
    pub fn new() -> Self {
        Storm { }
    }

    pub fn setup_container(&self, docker: &mut MultiContainer) {
        docker.set_compose("lib/TrackBench.rs/compose-storm.yaml");
    }
}

