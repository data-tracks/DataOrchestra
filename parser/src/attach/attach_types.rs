use serde::{Deserialize, Serialize};
use data_orchestra_engine::object::Object;
use crate::traits::Creatable;
use crate::types::object::ExtObject;
use super::types::{kafka_consumer::KafkaConsumer, kafka_producer::KafkaProducer};

/// Attachable types to a [`Object`]. Allow for easy creation of systems without having to manually
/// create adapters for systems
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AttachType {
    KafkaProducer,
    KafkaConsumer,
}

/// Configuration of attachable types from [`AttachType`]
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AttachTypeConfig {
    KafkaConsumer(KafkaConsumer),
    KafkaProducer(KafkaProducer),
}

impl Default for AttachTypeConfig {
    fn default() -> Self {
        Self::KafkaProducer(KafkaProducer::default())
    }
}

impl Creatable<ExtObject, Object> for AttachTypeConfig {
    fn create(self, object: &ExtObject) -> Object {
        match self {
            AttachTypeConfig::KafkaConsumer(consumer) => consumer.create(object),
            AttachTypeConfig::KafkaProducer(producer) => producer.create(object),
        }
    }
}
