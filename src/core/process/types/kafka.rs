use log::{info, error};
use serde::{Deserialize, Serialize};
use crate::core::adapters::docker::ComposeGroupBuilder;
use crate::core::adapters::Runner;

/// The `Kafka` type. Represents the configurability of the Apache kafka application instance
#[derive(Debug, Serialize, Deserialize)]
pub struct Kafka {
    topics: Vec<String>
}


impl Kafka {
    pub fn new() -> Self {
        Kafka { topics: Vec::new() }
    }

    /// Sets up the docker [`ComposeGroupBuilder`] with the configuration specific to the
    /// kafka application
    pub fn setup_container(&self, docker: &mut ComposeGroupBuilder) {
        docker.set_compose("images/compose-kafka.yaml");
    }

    /// Create kafka topics for the broker of the [`Kafka`] `topics` field
    ///
    /// Runner should the executor at the location of the docker deamon due to the `docker exec`
    /// command execution
    pub fn create_topic(&self, id: &String, runner: &Box<dyn Runner + Send>) {
        info!("Creating kafka topics {:?}", self.topics);
        for topic in &self.topics {
            self.exec_create_topic(id, topic, runner);
        }
    }

    fn exec_create_topic<T: Into<String>>(&self, id: &String, topic: T, runner: &Box<dyn Runner + Send>) {
        let result = runner.exec(format!("docker exec {} /opt/kafka/bin/kafka-topics.sh --bootstrap-server localhost:9092 --create --topic {}", id, topic.into()));
        if let Err(error) = result {
            error!("{}", error);
        }
    }
}

