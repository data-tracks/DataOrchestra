use serde::{Deserialize, Serialize};

use crate::{core::{object::Object, traits::Creator}, interface::general::General};

use super::types::{kafka_consumer::KafkaConsumer, kafka_producer::KafkaProducer};

/// Attachable types to a [`Object`]. Allow for easy creation of systems without having to manually
/// create adapters for systems
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AttachType {
    KafkaProducer,
    KafkaConsumer
}

/// Configuration of attachable types from [`AttachType`]
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AttachTypeConfig {
    KafkaConsumer(KafkaConsumer),
    KafkaProducer(KafkaProducer)
}

impl Default for AttachTypeConfig {
    fn default() -> Self {
        Self::KafkaProducer(KafkaProducer::default())
    }
}

impl Creator<Object> for AttachTypeConfig {
    fn create(self, general: &General) ->  Object {
        match self {
            AttachTypeConfig::KafkaConsumer(consumer) => consumer.create(general),
            AttachTypeConfig::KafkaProducer(producer) => producer.create(general),
        }
    }
}
