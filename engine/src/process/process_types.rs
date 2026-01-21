use super::{
    Process,
    types::{Flink, Kafka, Spark, Storm},
};
use crate::traits::Configurator;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Types of pre configured processes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProcessType {
    Flink,
    Kafka,
    Spark,
    Storm,
}

impl Display for ProcessType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ProcessType::Flink => String::from("Flink"),
            ProcessType::Kafka => String::from("Kafka"),
            ProcessType::Spark => String::from("Spark"),
            ProcessType::Storm => String::from("Storm"),
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
            ProcessType::Storm => ProcessTypeConfig::Storm(Storm::new()),
        }
    }
}

/// Configuration for process types in [`ProcessType`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProcessTypeConfig {
    Flink(Flink),
    Kafka(Kafka),
    Spark(Spark),
    Storm(Storm),
}

impl Configurator<Process> for ProcessTypeConfig {
    fn configure(&mut self, parent: &mut Process) {
        match self {
            ProcessTypeConfig::Kafka(kafka) => kafka.configure(parent),
            ProcessTypeConfig::Flink(_flink) => panic!("Flink not implemented yet"),
            ProcessTypeConfig::Spark(_spark) => panic!("Spark implemented yet"),
            ProcessTypeConfig::Storm(_storm) => panic!("Storm not implemented yet"),
        }
    }
}
