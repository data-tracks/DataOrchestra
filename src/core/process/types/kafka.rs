use std::net::{IpAddr, Ipv4Addr};

use log::{info, error};
use serde::{Deserialize, Serialize};
use crate::core::adapters::docker::ComposeGroupBuilder;
use crate::core::adapters::Runner;
use crate::core::object::Object;
use crate::core::traits::Configurator;


/// The `Kafka` type. Represents the configurability of the Apache kafka application instance
#[derive(Debug, Serialize, Deserialize)]
pub struct Kafka {
    #[serde(default)]
    pub topics: Vec<String>,
    #[serde(default = "default_host")]
    pub host: IpAddr 
}

pub fn default_host() -> IpAddr {
    IpAddr::V4(Ipv4Addr::LOCALHOST)
}

impl Configurator for Kafka {
    fn configure(&mut self, object: &mut Object) {
        self.host = object.node.as_ref()
            .map(|n| n.host.to_owned())
            .unwrap_or(default_host());

        let compose = ComposeGroupBuilder::new()
            .compose("images/compose-kafka.yaml")
            .interpolation_variable("KAFKA_HOST", self.host.to_string())
            .name("kafka-broker")
            .name("kafka-rest");

        object.docker_group_builder = Some(compose);
    }
}



impl Default for Kafka {
    fn default() -> Self {
        Kafka 
        { 
            topics: Vec::new(), 
            host: IpAddr::V4(Ipv4Addr::LOCALHOST)
        }
    }
}

impl Kafka {
    pub fn new(topics: Vec<String>, host: IpAddr) -> Self {
        Kafka { topics, host }
    }

    /// Create kafka topics for the broker of the [`Kafka`] `topics` field
    ///
    /// Runner should the executor at the location of the docker deamon due to the `docker exec`
    /// command execution
    pub fn create_topic(&self, id: &String, runner: &dyn Runner) {
        info!("Creating kafka topics {:?}", self.topics);
        for topic in &self.topics {
            self.exec_create_topic(id, topic, runner);
        }
    }

    fn exec_create_topic<T: Into<String>>(&self, id: &String, topic: T, runner: &dyn Runner) {
        let result = runner.exec(format!("docker exec {} /opt/kafka/bin/kafka-topics.sh --bootstrap-server localhost:9092 --create --topic {}", id, topic.into()));
        if let Err(error) = result {
            error!("{}", error);
        }
    }
}

