use log::LevelFilter;
use serde::{Deserialize, Serialize};
use crate::logger::{deserialize_levelfilter, serialize_levelfilter};

use crate::core::types::data::{DockerData, VolatileDockerData};
use crate::shared::ToInternal;
use crate::interface::{general::General, object::ExtObject};
use crate::core::adapters::ContainerBuilder;
use crate::core::object::Object;
use crate::core::attach::attach_types::ToObject;

// The Kafka consumer type. Is an attachable object capable of consuming data from kafka topic(s)
// and sending them further through a http request
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KafkaConsumer {
    // External Object type to allow for the configuration of the consumer.
    // Set to option as it would otherwise overflow the stack due to circular dependency
    pub object: Option<Box<ExtObject>>,
    #[serde(flatten)]
    pub args: Arguments
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Arguments {
    /// Address where data should be send to
    pub address: String,    
    /// Address (host:port) of kafka 
    #[serde(default = "default_consumer")]
    pub consumer: String,
    /// Group id of consumer
    pub group_id: String,
    /// Kafka topics consumer should consume from
    pub topics: Vec<String>,
    /// Logging level
    #[serde(deserialize_with = "deserialize_levelfilter")]
    #[serde(serialize_with = "serialize_levelfilter")]
    #[serde(default = "default_level")]
    pub level: LevelFilter
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments { address: "".to_string(), consumer: default_consumer(), group_id: "".to_string(), topics: Vec::new(), level: default_level() }
    }
}


pub fn default_consumer() -> String {
    "localhost:9092".to_string()
}

pub fn default_level() -> LevelFilter {
    LevelFilter::Info
}

impl Default for KafkaConsumer {
    fn default() -> Self {
        KafkaConsumer 
        { 
            object: None,
            args: Arguments::default()
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

        object.name = "kafka-consumer".to_string();
        object.graph.ignore = true;

        if let Some(node) = general.node {
            object.node = Some(node.to_internal());
        }

        object.docker_data.push(DockerData::new
            (
                "", 
                "attachables/kafka_consumer/", 
                "/kafka_consumer", 
                "/kafka_consumer/start.sh", 
                None,
                None
            )
        );

        object.docker_container_builder.get_or_insert(ContainerBuilder::new());

        if let Some(builder) = object.docker_container_builder.as_mut() {
            builder
                .try_name_mut("kafka-consumer")
                .dockerfile_mut("images/rust.dockerfile")
                .image_mut("rust_base");
        }

        let name = object.docker_container_builder
            .as_ref()
            .unwrap()
            .get_name()
            .unwrap();

        let json = serde_json::to_string_pretty(&self.args).expect("Unable to parse struct to json");

        object.docker_sftp_data.push(VolatileDockerData::new
            (
                name.to_owned(),
                "/kafka_consumer/config.json".into(),
                json
            )
        );

        object
    }
}
