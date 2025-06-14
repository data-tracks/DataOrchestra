use std::fmt::Display;
use serde::{Deserialize, Serialize};

use crate::core::adapters::docker::ComposeGroupBuilder;

use super::types::{Flink, Kafka, Spark, Storm};

/// Types of pre configured processes 
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProcessType {
    Flink,
    Kafka,
    Spark,
    Storm
}

impl Display for ProcessType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ProcessType::Flink => String::from("Flink"),
            ProcessType::Kafka => String::from("Kafka"),
            ProcessType::Spark => String::from("Spark"),
            ProcessType::Storm => String::from("Storm")
        };
        write!(f, "{}", str)
    }
}

//TODO: Rewrite because kafka::default is bad 
impl ProcessType {
    pub fn new(&self) -> ProcessTypeConfig {
        match self {
            ProcessType::Flink => ProcessTypeConfig::Flink(Flink::new()),
            ProcessType::Kafka => ProcessTypeConfig::Kafka(Kafka::default()),
            ProcessType::Spark => ProcessTypeConfig::Spark(Spark::new()),
            ProcessType::Storm => ProcessTypeConfig::Storm(Storm::new())
        }
    }
}

/// Configuration for process types in [`ProcessType`]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProcessTypeConfig {
    Flink(Flink),
    Kafka(Kafka),
    Spark(Spark),
    Storm(Storm)
}

impl ProcessTypeConfig {
    /// Setup the given docker container with config of specified [`ProcessType`]
    pub fn setup_container(&self, docker: &mut ComposeGroupBuilder) {
        match self {
            ProcessTypeConfig::Flink(flink) => flink.setup_container(docker),
            ProcessTypeConfig::Kafka(kafka) => kafka.setup_container(docker),
            ProcessTypeConfig::Spark(spark) => spark.setup_container(docker),
            ProcessTypeConfig::Storm(storm) => storm.setup_container(docker)
        }
    }
}

