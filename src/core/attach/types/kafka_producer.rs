use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use tracing_subscriber::filter::LevelFilter;

use crate::core::object::Object;
use crate::core::traits::Creator;
use crate::core::types::data::{DataBuilder, DataTypes, VolatileDataBuilder};
use crate::core::types::{Executables, ScriptBuilder};
use crate::interface::general::General;
use crate::logger::{deserialize_levelfilter, serialize_levelfilter};
use crate::shared::ToInternal;

// The Kafka producer type. Is an attachable object capable of producing data to kafka topic(s)
#[derive(Debug, Deserialize, Serialize, Clone, Builder)]
pub struct KafkaProducer {
    #[serde(default = "default_api_port")]
    pub api_port: u16,

    #[serde(default = "default_address")]
    #[builder(setter(into))]
    pub address: String,

    #[serde(deserialize_with = "deserialize_levelfilter")]
    #[serde(serialize_with = "serialize_levelfilter")]
    #[serde(default = "default_level")]
    #[builder(default = "default_level()")]
    pub level: LevelFilter,

    pub topics: Vec<String>,

    #[serde(default)]
    #[builder(default)]
    pub logger: Option<String>,
}

pub fn default_api_port() -> u16 {
    5000
}

pub fn default_address() -> String {
    "localhost:9092".to_string()
}

pub fn default_level() -> LevelFilter {
    LevelFilter::INFO
}

impl KafkaProducerBuilder {
    pub fn topic(&mut self, topic: impl Into<String>) -> &mut Self {
        if self.topics.is_none() {
            self.topics = Some(Vec::new());
        }
        let topics = self.topics.as_mut().unwrap();
        topics.push(topic.into());
        self
    }
}

impl Default for KafkaProducer {
    fn default() -> Self {
        KafkaProducer {
            api_port: default_api_port(),
            address: default_address(),
            level: default_level(),
            topics: Vec::new(),
            logger: None,
        }
    }
}

impl Creator<Object> for KafkaProducer {
    fn create(self, general: &General) -> Object {
        // Clone due to the general struct later also being used to parse the actual object where
        // the attach object is attached to
        let general = general.clone();

        let mut object = Object::default();

        object.name = general.name.unwrap_or("kafka-producer".to_string());

        if let Some(node) = general.node {
            object.node = Some(node.to_internal());
        }

        object.graph.ignore = true;

        let docker_data = DataBuilder::default()
            .source("services/attachables/kafka_producer")
            .destination("/kafka_producer")
            .build()
            .expect("Unable to build docker_data");

        let script = ScriptBuilder::default()
            .path("/kafka_producer/start.sh")
            .build()
            .expect("Unable to build script");

        object.resources.push(DataTypes::DockerData(docker_data));

        object.docker_container_builder.get_or_insert_default();

        if let Some(builder) = object.docker_container_builder.as_mut() {
            builder
                .try_name("kafka-producer")
                .dockerfile("images/rust.dockerfile")
                .image("rust_base")
                .publish(self.api_port);
        }

        let json = serde_json::to_string_pretty(&self).expect("Unable to parse struct to json");

        let volatile_data = VolatileDataBuilder::default()
            .destination("/kafka_producer/config.json")
            .content(json)
            .build()
            .expect("Unable to build volatile data");

        object
            .resources
            .push(DataTypes::VolatileDockerData(volatile_data));
        object.executables.push(Executables::Script(script));

        object
    }
}
