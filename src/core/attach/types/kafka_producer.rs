use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::core::adapters::ContainerBuilder;
use crate::interface::general::General;
use crate::core::types::DockerData;
use crate::core::object::Object;
use crate::core::attach::attach_types::ToObject;
use crate::interface::object::ExtObject;
use crate::shared::ToInternal;

// The Kafka consumer type. Is an attachable object capable of consuming data from kafka topic(s)
// and sending them further through a http request
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KafkaProducer {
    // Topics the producer writes to
    pub topics: Vec<String>,
    // External Object type to allow for the configuration of the producer
    pub object: Option<Box<ExtObject>>,
    #[serde(default = "default_api_port")]
    pub api_port: u16,
    #[serde(default = "default_level")]
    pub level: String,
    pub kafka_address: String
}

pub fn default_api_port() -> u16 {
    8080
}

pub fn default_level() -> String {
    "info".to_string()
}

impl Default for KafkaProducer {
    fn default() -> Self {
        KafkaProducer 
        { 
            topics: Vec::new(), 
            object: None,
            api_port: default_api_port(),
            level: default_level(),
            kafka_address: "localhost:9092".to_string()
        }
    }
}


impl ToObject for KafkaProducer {
    fn to_object(self, general: &General) -> Object {
        // Clone due to the general struct later also being used to parse the actual object where
        // the attach object is attached to
        let general = general.clone();

        let mut object = Object::default();
        if let Some(ext_object) = self.object {
            object = ext_object.to_internal();
        }

        object.name = "kafka-producer".to_string();

        if let Some(node) = general.node {
            object.node = Some(node.to_internal());
        }

        object.docker_data.push(DockerData::new
            (
                "", 
                "attachables/kafka_producer/", 
                "kafka_producer/", 
                "kafka_producer/start.sh", 
                None,
                None
            )
        );
        
        object.docker_container_builder.get_or_insert(ContainerBuilder::new());

        if let Some(builder) = object.docker_container_builder.as_mut() {
            builder
                .try_name_mut("KafkaProducer")
                .dockerfile_mut("images/rust.dockerfile")
                .image_mut("rust_base");
        }

        object
    }
}
