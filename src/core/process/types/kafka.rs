use log::info;
use serde::{Deserialize, Serialize};
use crate::core::adapters::command::command_func::output_command;
use crate::core::adapters::docker::ComposeGroupBuilder;

#[derive(Debug, Serialize, Deserialize)]
pub struct Kafka {
    topics: Vec<String>
}


impl Kafka {
    pub fn new() -> Self {
        Kafka { topics: Vec::new() }
    }

    pub fn setup_container(&self, docker: &mut ComposeGroupBuilder) {
        docker.set_compose("lib/TrackBench.rs/compose-kafka.yaml");
    }

    pub fn create_topic(&self, id: &String) {
        info!("Creating kafka topics {:?}", self.topics);
        for topic in &self.topics {
            self.exec_create_topic(id, topic);
        }
    }

    fn exec_create_topic<T: Into<String>>(&self, id: &String, topic: T) {
        let _ = output_command(format!("docker exec {} /opt/kafka/bin/kafka-topics.sh --bootstrap-server localhost:9092 --create --topic {}", id, topic.into()));
    }
}

