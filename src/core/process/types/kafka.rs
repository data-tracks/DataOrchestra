use crate::core::adapters::docker::DockerPlan;
use crate::shared::Amount;
use crate::core::adapters::ssh::Ssh;

#[derive(Debug)]
pub struct Kafka {
    topics: Amount<String>
}


impl Kafka {
    pub fn new() -> Self {
        Kafka { topics: Amount::None }
    }

    pub fn setup_container(&self, docker: &mut DockerPlan) {
        docker.set_compose("lib/TrackBench.rs/compose-kafka.yaml");
    }

    pub fn create_topic(&self, ssh: &Ssh) {
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

    fn exec_create_topic<T: Into<String>>(&self, ssh: &Ssh, topic: T) {
       ssh.exec(format!("/opt/kafka/bin/kafka-topics.sh --bootstrap-server localhost:9092 --create --topic {}", topic.into())); 
    }
}

