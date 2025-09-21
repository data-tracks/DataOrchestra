use derive_builder::Builder;
use tracing_subscriber::filter::LevelFilter;
use serde::{Deserialize, Serialize};

use crate::core::traits::Creator;
use crate::core::types::{Executables, ScriptBuilder};
use crate::logger::{deserialize_levelfilter, serialize_levelfilter};
use crate::core::types::data::{DataBuilder, DataTypes, VolatileDataBuilder};
use crate::shared::ToInternal;
use crate::core::object::Object;
use crate::interface::object::ExtObject;

// The Kafka consumer type. Is an attachable object capable of consuming data from kafka topic(s)
// and sending them further through an http request
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KafkaConsumer {
    #[serde(flatten)]
    pub args: Arguments
}


#[derive(Debug, Clone, Deserialize, Serialize, Builder)]
pub struct Arguments {
    /// Address where data should be sent to
    #[builder(setter(into))]
    pub address: String,    
    /// Address (host:port) of kafka 
    #[serde(default = "default_consumer")]
    #[builder(default = "default_consumer()")]
    #[builder(setter(into))]
    pub consumer: String,
    /// Group id of consumer
    #[builder(setter(into))]
    pub group_id: String,
    /// Kafka topics consumer should consume from
    pub topics: Vec<String>,
    /// Logging level
    #[serde(deserialize_with = "deserialize_levelfilter")]
    #[serde(serialize_with = "serialize_levelfilter")]
    #[serde(default = "default_level")]
    #[builder(default = "default_level()")]
    pub level: LevelFilter,

    #[serde(default)]
    #[builder(default)]
    pub logger: Option<String>
}



impl ArgumentsBuilder {
    pub fn topic(&mut self, topic: impl Into<String>) -> &mut Self {
        if self.topics.is_none() {
            self.topics = Some(Vec::new());
        }
        let topics = self.topics.as_mut().unwrap();
        topics.push(topic.into());
        self
    }
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments {  address: "".to_string(), consumer: default_consumer(), group_id: "".to_string(), topics: Vec::new(), level: default_level(), logger: None }
    }
}

pub fn default_consumer() -> String {
    "localhost:9092".to_string()
}

pub fn default_level() -> LevelFilter {
    LevelFilter::INFO
}

impl Default for KafkaConsumer {
    fn default() -> Self {
        KafkaConsumer 
        { 
            args: Arguments::default()
        }
    }
}

impl Creator<Object> for KafkaConsumer {
    fn create(self, parent_object: &ExtObject) -> Object {
        // Clone due to the general struct later also being used to parse the actual object where
        // the attach object is attached to
        let parent_object = parent_object.clone();

        let mut object = Object::default();

        object.name = parent_object.name.unwrap_or("kafka-consumer".to_string());

        if let Some(node) = parent_object.node {
            let (node, uploader) = node.to_internal();
            object.node = Some(node);
            object.uploader = Some(uploader);
        }

        object.graph.ignore = true;

        let docker_data = DataBuilder::default()
            .src("services/attachables/kafka_consumer")
            .dst("/kafka_consumer")
            .build()
            .expect("Unable to build docker_data");

        let script = ScriptBuilder::default()
            .path("/kafka_consumer/start.sh")
            .build()
            .expect("Unable to build script");

        object.resources.push(DataTypes::Data(docker_data));

        object.docker_container_builder.get_or_insert_default();

        if let Some(builder) = object.docker_container_builder.as_mut() {
            builder
                .try_name("kafka-consumer")
                .dockerfile("images/rust.dockerfile")
                .image("rust_base");
        }

        let json = serde_json::to_string_pretty(&self.args).expect("Unable to parse struct to json");

        let volatile_data = VolatileDataBuilder::default()
            .dst("/kafka_consumer/config.json")
            .content(json)
            .build()
            .expect("Unable to build volatile data");

        object.resources.push(DataTypes::VolatileData(volatile_data));
        object.executables.push(Executables::Script(script));
        
        object
    }
}
