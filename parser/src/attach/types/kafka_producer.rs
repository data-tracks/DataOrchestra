use serde::{Deserialize, Serialize};
use tracing_subscriber::filter::LevelFilter;

use crate::interface::object::ExtObject;
use crate::logger::{deserialize_levelfilter, serialize_levelfilter};
use crate::object::Object;
use crate::shared::ToInternal;
use crate::traits::Creator;
use crate::types::data::{DataBuilder, DataTypes, VolatileDataBuilder};
use crate::types::{Executables, ScriptBuilder};

// The Kafka producer type. Is an attachable object capable of producing data to kafka topic(s)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KafkaProducer {
    #[serde(flatten)]
    pub args: Arguments,
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

    pub topics: Vec<String>,

    #[serde(default)]
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

impl Default for Arguments {
    fn default() -> Self {
        Arguments {
            api_port: default_api_port(),
            address: default_address(),
            level: default_level(),
            topics: Vec::new(),
            logger: None,
        }
    }
}

impl Default for KafkaProducer {
    fn default() -> Self {
        KafkaProducer {
            args: Arguments::default(),
        }
    }
}

impl Creator<ExtObject, Object> for KafkaProducer {
    fn create(self, parent_object: &ExtObject) -> Object {
        // Clone due to the general struct later also being used to parse the actual object where
        // the attach object is attached to
        let parent_object = parent_object.clone();

        let mut object = Object::default();

        object.name = parent_object.name.unwrap_or("kafka-producer".to_string());

        if let Some(node) = parent_object.node {
            let (node, uploader) = node.to_internal();
            object.node = Some(node);
            object.uploader = Some(uploader);
        }

        object.graph.ignore = true;

        let docker_data = DataBuilder::default()
            .src("services/attachables/kafka_producer")
            .dst("/kafka_producer")
            .build()
            .expect("Unable to build docker_data");

        let script = ScriptBuilder::default()
            .path("/kafka_producer/start.sh")
            .build()
            .expect("Unable to build script");

        object.resources.push(DataTypes::Data(docker_data));

        object.docker_container_builder.get_or_insert_default();

        if let Some(builder) = object.docker_container_builder.as_mut() {
            builder
                .try_name("kafka-producer")
                .dockerfile("images/rust.dockerfile")
                .image("rust_base")
                .publish(self.args.api_port);
        }

        let json =
            serde_json::to_string_pretty(&self.args).expect("Unable to parse struct to json");

        let volatile_data = VolatileDataBuilder::default()
            .dst("/kafka_producer/config.json")
            .content(json)
            .build()
            .expect("Unable to build volatile data");

        object
            .resources
            .push(DataTypes::VolatileData(volatile_data));
        object.executables.push(Executables::Script(script));

        object
    }
}
