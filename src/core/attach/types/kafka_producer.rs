use crate::core::types::data::VolatileDockerData;
use crate::logger::{serialize_levelfilter, deserialize_levelfilter};

use log::LevelFilter;
use serde::{Deserialize, Serialize};

use crate::core::adapters::ContainerBuilder;
use crate::interface::general::General;
use crate::core::types::DockerData;
use crate::core::object::Object;
use crate::core::attach::attach_types::ToObject;
use crate::shared::ToInternal;

// The Kafka producer type. Is an attachable object capable of producing data to kafka topic(s)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KafkaProducer {
    #[serde(flatten)]
    pub args: Arguments
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Arguments {
    #[serde(default = "default_api_port")]
    pub api_port: u16,

    #[serde(default = "default_address")]
    pub address: String,

    #[serde(deserialize_with = "deserialize_levelfilter")]
    #[serde(serialize_with = "serialize_levelfilter")]
    #[serde(default = "default_level")]
    pub level: LevelFilter,

    pub topics: Vec<String>
}

pub fn default_api_port() -> u16 {
    5000
}

pub fn default_address() -> String {
    "localhost:9092".to_string()
}

pub fn default_level() -> LevelFilter {
    LevelFilter::Info
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments 
        { 
            api_port: 5000, 
            address: "localhost:9092".to_string(), 
            level: LevelFilter::Info, 
            topics: Vec::new() 
        }
    }
}

impl Default for KafkaProducer {
    fn default() -> Self {
        KafkaProducer 
        { 
            args: Arguments::default()
        }
    }
}

impl ToObject for KafkaProducer {
    fn to_object(self, general: &General) -> Object {
        // Clone due to the general struct later also being used to parse the actual object where
        // the attach object is attached to
        let general = general.clone();

        let mut object = Object::default();

        object.name = general.name.unwrap_or("kafka-producer".to_string());

        if let Some(node) = general.node {
            object.node = Some(node.to_internal());
        }

        object.graph.ignore = true;

        object.docker_data.push(DockerData::new
            (
                "", 
                "services/attachables/kafka_producer", 
                "/kafka_producer", 
                "/kafka_producer/start.sh", 
                None,
                None
            )
        );
        
        object.docker_container_builder.get_or_insert(ContainerBuilder::new());

        if let Some(builder) = object.docker_container_builder.as_mut() {
            builder
                .try_name_mut("kafka-producer")
                .dockerfile_mut("images/rust.dockerfile")
                .image_mut("rust_base")
                .publish_mut(self.args.api_port);
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
                "/kafka_producer/config.json".into(),
                json
            )
        );

        object
    }
}
