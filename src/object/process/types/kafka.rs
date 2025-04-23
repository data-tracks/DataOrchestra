use serde::{Deserialize, Serialize};

use crate::{docker::docker_struct::Container, ssh::ssh_struct::ssh, types::amount::Amount};

#[derive(Debug, Deserialize, Serialize)]
pub struct Kafka {
    #[serde(default)]
    topics: Amount<String>
}


impl Kafka {
    pub fn new() -> Self {
        Kafka { topics: Amount::None }
    }

    pub fn setup_container(&self, docker: &mut Container) {
        docker.set_compose("lib/TrackBench.rs/compose-kafka.yaml");
    }

    pub fn create_topic(&self, ssh: &ssh) {
        match self.topics {
            Amount::Single(ref topic) => self.exec_create_topic(ssh, topic),
            Amount::Multiple(ref topics) => {
                for topic in topics {
                    self.exec_create_topic(ssh, topic);
                }
            },
            Amount::None => (),
        };
    }

    fn exec_create_topic<T: Into<String>>(&self, ssh: &ssh, topic: T) {
       ssh.exec(format!("/opt/kafka/bin/kafka-topics.sh --bootstrap-server localhost:9092 --create --topic {}", topic.into())); 
    }
}

