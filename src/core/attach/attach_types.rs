use serde::{Deserialize, Serialize};

use crate::{
    core::{object::Object, traits::Creator},
    interface::ext_object::ExtObject,
};

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

impl Creator<ExtObject, Object> for AttachTypeConfig {
    fn create(self, parent: &ExtObject) -> Object {
        match self {
            AttachTypeConfig::KafkaConsumer(consumer) => consumer.create(parent),
            AttachTypeConfig::KafkaProducer(producer) => producer.create(parent),
        }
    }
}
