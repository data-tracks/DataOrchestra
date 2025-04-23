use serde::{Deserialize, Serialize};

use crate::docker::docker_struct::Container;

use super::types::{flink::Flink, kafka::Kafka, spark::Spark, storm::Storm};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProcessType {
    Flink,
    Kafka,
    Spark,
    Storm
}

impl ProcessType {
    pub fn new(&self) -> ProcessTypeConfig {
        match self {
            ProcessType::Flink => ProcessTypeConfig::Flink(Flink::new()),
            ProcessType::Kafka => ProcessTypeConfig::Kafka(Kafka::new()),
            ProcessType::Spark => ProcessTypeConfig::Spark(Spark::new()),
            ProcessType::Storm => ProcessTypeConfig::Storm(Storm::new())
        }
    }
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ProcessTypeConfig {
    Flink(Flink),
    Kafka(Kafka),
    Spark(Spark),
    Storm(Storm)
}

impl ProcessTypeConfig {
    /// Setup the given docker container with config of specified [`ProcessType`]
    /// Consumes the docker container object and returns the modified container
    pub fn setup_container(&self, docker: &mut Container) {
        match self {
            ProcessTypeConfig::Flink(flink) => flink.setup_container(docker),
            ProcessTypeConfig::Kafka(kafka) => kafka.setup_container(docker),
            ProcessTypeConfig::Spark(spark) => spark.setup_container(docker),
            ProcessTypeConfig::Storm(storm) => storm.setup_container(docker)
        }
    }
}

