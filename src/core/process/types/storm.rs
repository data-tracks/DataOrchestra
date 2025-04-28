use crate::core::adapters::docker::DockerPlan;

#[derive(Debug)]
pub struct Storm {}

impl Storm {
    pub fn new() -> Self {
        Storm { }
    }

    pub fn setup_container(&self, docker: &mut DockerPlan) {
        docker.set_compose("lib/TrackBench.rs/compose-storm.yaml");
    }
}

