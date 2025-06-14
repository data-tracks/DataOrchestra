use serde::{Deserialize, Serialize};

use crate::{core::object::Object, interface::general::General};

use super::types::{kafka_consumer::KafkaConsumer, kafka_producer::KafkaProducer};

/// Attachable types to a [`Object`]. Allow for easy creation of systems without having to manually
/// create adapters for systems
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AttachType {
    KafkaProducer,
    KafkaConsumer
}

/// Configuration of attachable types from [`AttachType`]
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AttachTypeConfig {
    KafkaConsumer(KafkaConsumer),
    KafkaProducer(KafkaProducer)
}

impl Default for AttachTypeConfig {
    fn default() -> Self {
        Self::KafkaProducer(KafkaProducer::default())
    }
}

pub trait ToObject {
    fn to_object(self, general: &General) -> Object;
}

impl ToObject for AttachTypeConfig {
    fn to_object(self, general: &General) -> Object {
        match self {
            AttachTypeConfig::KafkaConsumer(consumer) => consumer.to_object(general),
            AttachTypeConfig::KafkaProducer(producer) => producer.to_object(general),
        }
    } 
}
