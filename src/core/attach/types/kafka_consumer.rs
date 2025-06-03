use serde::{Deserialize, Serialize};

use crate::core::types::data::DockerData;
use crate::shared::ToInternal;
use crate::interface::{general::General, object::ExtObject};
use crate::core::adapters::ContainerBuilder;
use crate::core::object::Object;
use crate::core::attach::attach_types::ToObject;

// The Kafka consumer type. Is an attachable object capable of consuming data from kafka topic(s)
// and sending them further through a http request
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KafkaConsumer {
    // Topics the consumer reads from
    pub topics: Vec<String>,
    // External Object type to allow for the configuration of the consumer.
    // Set to option as it would otherwise overflow the stack due to circular dependency
    pub object: Option<Box<ExtObject>>,
    // Re exports of arguments
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

impl Default for KafkaConsumer {
    fn default() -> Self {
        KafkaConsumer 
        { 
            topics: Vec::new(), 
            object: None,
            api_port: default_api_port(),
            level: default_level(),
            kafka_address: "localhost:9092".to_string()
        }
    }
}

impl ToObject for KafkaConsumer {
    fn to_object(self, general: &General) -> Object {
        // Clone due to the general struct later also being used to parse the actual object where
        // the attach object is attached to
        let general = general.clone();

        let mut object = Object::default();
        if let Some(ext_object) = self.object {
            object = ext_object.to_internal();
        }

        if let Some(node) = general.node {
            object.node = Some(node.to_internal());
        }

        let env = 

        object.docker_data.push(DockerData::new
            (
                "", 
                "attachables/kafka_consumer/", 
                "kafka_consumer/", 
                "kafka_consumer/start.sh", 
                None,
                None)
            );
        
        object.docker_container_builder.get_or_insert(ContainerBuilder::new());

        if let Some(builder) = object.docker_container_builder.as_mut() {
            builder
                .try_set_name("KafkaConsumer")
                .set_dockerfile("images/rust.dockerfile")
                .set_image("rust_base");
        }

        object
    }
}
