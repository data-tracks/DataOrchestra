use log::{info, error};
use serde::{Deserialize, Serialize};
use crate::core::adapters::docker::ComposeGroupBuilder;
use crate::core::adapters::Runner;

#[derive(Debug, Serialize, Deserialize)]
pub struct Kafka {
    topics: Vec<String>
}


impl Kafka {
    pub fn new() -> Self {
        Kafka { topics: Vec::new() }
    }

    pub fn setup_container(&self, docker: &mut ComposeGroupBuilder) {
        docker.set_compose("images/compose-kafka.yaml");
    }

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

