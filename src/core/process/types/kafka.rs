use std::net::{IpAddr, Ipv4Addr};

use log::{error, info};
use serde::{Deserialize, Serialize};
use crate::core::adapters::Executor;
use crate::core::process::Process;
use crate::core::traits::Configurator;


/// The `Kafka` type. Represents the configurability of the Apache kafka application instance
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Kafka {
    #[serde(default)]
    pub topics: Vec<String>,
    #[serde(default = "default_host")]
    pub host: IpAddr 
}

pub fn default_host() -> IpAddr {
    IpAddr::V4(Ipv4Addr::LOCALHOST)
}

impl Configurator<Process> for Kafka {
    fn configure(&mut self, parent: &mut Process) {
        self.host = parent.object.node.as_ref()
            .map(|n| n.host.to_owned())
            .unwrap_or(default_host());

        parent.object.docker_group_builder.as_mut().unwrap()
            .compose("images/compose-kafka.yaml")
            .interpolation_variable("KAFKA_HOST", self.host.to_string());
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
    /// Executor should be the executor at the location of the docker daemon due to the `docker exec`
    /// command execution
    pub fn create_topic(&self, id: &String, executor: &dyn Executor) {
        info!("Creating kafka topics {:?}", self.topics);
        for topic in &self.topics {
            self.exec_create_topic(id, topic, executor);
        }
    }

    fn exec_create_topic<T: Into<String>>(&self, id: &String, topic: T, executor: &dyn Executor) {
        let result = executor.exec(format!("docker exec {} /opt/kafka/bin/kafka-topics.sh --bootstrap-server localhost:9092 --create --topic {}", id, topic.into()));
        if let Err(error) = result {
            error!("{}", error);
        }
    }
}

