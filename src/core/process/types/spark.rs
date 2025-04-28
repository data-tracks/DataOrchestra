use crate::core::adapters::docker::DockerPlan;

#[derive(Debug)]
pub struct Spark {}

impl Spark {
    pub fn new() -> Self {
        Spark { }
    }

    pub fn setup_container(&self, docker: &mut DockerPlan) {
        docker.set_compose("lib/TrackBench.rs/compose-spark.yaml");
    }
}

