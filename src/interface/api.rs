use serde::{Deserialize, Serialize};

use crate::core::traits::Creator;
use crate::core::process::{process_types::ProcessTypeConfig, types::Kafka, Process};
use crate::core::object::Object;
use crate::core::attach::types::kafka_consumer::KafkaConsumer;
use crate::core::attach::types::kafka_consumer::ArgumentsBuilder;
use crate::interface::object::ExtObject;
use crate::shared::ToInternal;
use super::node::ExtNode;

#[derive(Debug, Deserialize, Serialize)]
pub struct API {
    #[serde(default = "API::default_port")]
    port: u16,
    kafka_host: ExtNode 
}

impl API {
    pub fn default_port() -> u16 {
        5000
    }
}



impl ToInternal<(Process, Object)> for API {
    fn to_internal(self) -> (Process, Object) {
        let docker_name = "orchestra-api".to_string();

        // Build the api object
        let args = ArgumentsBuilder::default()
            .address(format!("http://host.docker.internal:{}/orchestra/broadcast", self.port))
            .consumer(format!("{}:9092", self.kafka_host.host.clone()))
            .topic("orchestra-log")
            .group_id("logger")
            .build()
            .expect("Unable to build arguments");

        // Inject consumer which consumes from kafka and sends it to the API
        let consumer = KafkaConsumer
        {
            args
        };

        let object = ExtObject
        {
            name: Some("kafka-api-consumer".to_string()),
            ..ExtObject::default()
        };

        let consumer = consumer.create(&object);

        let mut process = Process::default();

        process.object.name = "kafka-api".to_string();
        process.object.graph.ignore = true;

        let kafka = Kafka::new(vec!["orchestra-log".to_string()], self.kafka_host.host);
        process.config = Some(ProcessTypeConfig::Kafka(kafka));


        let (node, uploader) = self.kafka_host.to_internal();
        process.object.node = Some(node);
        process.object.uploader = Some(uploader);

        (process, consumer)
    } 
}
