use crate::core::adapters::docker::ComposeGroupBuilder;

#[derive(Debug)]
pub struct Flink {}

impl Flink {
    pub fn new() -> Self {
        Flink { }
    }

    pub fn setup_container(&self, docker: &mut ComposeGroupBuilder) {
        docker.set_compose("lib/TrackBench.rs/compose-flink.yaml");
    }
}




