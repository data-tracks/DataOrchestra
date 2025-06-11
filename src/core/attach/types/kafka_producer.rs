use crate::core::types::data::DockerSFTPData;
use crate::logger::{serialize_levelfilter, deserialize_levelfilter};

use log::LevelFilter;
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
    // External Object type to allow for the configuration of the producer
    pub object: Option<Box<ExtObject>>,
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
            object: None,
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
        if let Some(ext_object) = self.object {
            object = ext_object.to_internal();
        }

        object.name = "kafka-producer".to_string();
        object.graph.ignore = true;

        if let Some(node) = general.node {
            object.node = Some(node.to_internal());
        }

        object.docker_data.push(DockerData::new
            (
                "", 
                "attachables/kafka_producer/", 
                "/kafka_producer", 
                "/kafka_producer/start.sh", 
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

        let name = object.docker_container_builder
            .as_ref()
            .unwrap()
            .get_name()
            .unwrap();

        let json = serde_json::to_string_pretty(&self.args).expect("Unable to parse struct to json");

        object.docker_sftp_data.push(DockerSFTPData::new
            (
                name.to_owned(),
                "/kafka_producer/config.json".into(),
                json
            )
        );

        object
    }
}
