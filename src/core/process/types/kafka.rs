use serde::{Deserialize, Serialize};
use crate::core::adapters::docker::ComposeGroupBuilder;
use crate::core::adapters::ssh::Ssh;

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

    pub fn create_topic(&self, ssh: &Ssh) {
        for topic in &self.topics {
            self.exec_create_topic(ssh, topic);
        }
    }

    fn exec_create_topic<T: Into<String>>(&self, ssh: &Ssh, topic: T) {
       ssh.exec(format!("/opt/kafka/bin/kafka-topics.sh --bootstrap-server localhost:9092 --create --topic {}", topic.into())); 
    }
}

