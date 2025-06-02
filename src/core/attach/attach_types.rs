use serde::{Deserialize, Serialize};

use crate::{core::object::Object, interface::{general::General, object::ExtObject}};

use super::types::{kafka_consumer::KafkaConsumer, kafka_producer::KafkaProducer};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AttachType {
    KafkaProducer,
    KafkaConsumer
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AttachTypeConfig {
    KafkaConsumer(KafkaConsumer),
    KafkaProducer(KafkaProducer)
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
