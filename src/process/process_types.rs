use serde::{Deserialize, Serialize};

use super::types::{flink::Flink, kafka::Kafka, spark::Spark, storm::Storm};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProcessType {
    Flink,
    Kafka,
    Spark,
    Storm
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ProcessTypeConfig {
    Flink(Flink),
    Kafka(Kafka),
    Spark(Spark),
    Storm(Storm)
}
