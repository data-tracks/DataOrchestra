use serde::{Deserialize, Serialize};
use serde_json::json;

use super::node::ExtNode;
use crate::core::adapters::{ContainerBuilder, TmuxBuilder};
use crate::core::attach::types::kafka_consumer::ArgumentsBuilder;
use crate::core::attach::types::kafka_consumer::KafkaConsumer;
use crate::core::object::{Object, ObjectBuilder};
use crate::core::traits::Creator;
use crate::core::types::data::{DataBuilder, VolatileDataBuilder};
use crate::core::types::{Kafka, ScriptBuilder, ServiceConfig};
use crate::interface::ext_object::ExtObject;
use crate::shared::ToInternal;

#[derive(Debug, Deserialize, Serialize)]
pub struct API {
    #[serde(default = "default_port")]
    port: u16,
    kafka_host: ExtNode,
}

pub fn default_port() -> u16 {
    5000
}

impl ToInternal<(Object, Object)> for API {
    fn to_internal(self) -> (Object, Object) {
        let docker_name = "orchestra-api".to_string();

        // Build the api object
        let args = ArgumentsBuilder::default()
            .address(format!(
                "http://host.docker.internal:{}/orchestra/broadcast",
                self.port
            ))
            .consumer(format!("{}:9092", self.kafka_host.host.clone()))
            .topic("orchestra-log")
            .group_id("logger")
            .build()
            .expect("Unable to build arguments");

        let container = ContainerBuilder::default()
            .dockerfile("images/rust.dockerfile")
            .image("rust_base")
            .name("orchestra-api")
            .publish(self.port)
            .to_owned();

        let tmux = TmuxBuilder::default()
            .bash()
            .session("OrchestraApi")
            .command("cd /DataOrchestra/services/api")
            .command("cargo run")
            .build();

        let config = VolatileDataBuilder::default()
            .destination("/DataOrchestra/services/api/config.json")
            .content(json!({ "port": self.port }).to_string())
            .build()
            .expect("Unable to build config file");

        let shell = VolatileDataBuilder::default()
            .name(docker_name.clone())
            .destination("/DataOrchestra/services/api/start.sh")
            .content(tmux)
            .build()
            .expect("Unable to build shell script");

        let docker_data = DataBuilder::default()
            .source("../DataOrchestra")
            .destination("/DataOrchestra")
            .build()
            .expect("Unable to build docker data");

        let script = ScriptBuilder::default()
            .name(docker_name.clone())
            .path("/DataOrchestra/services/api/start.sh")
            .build()
            .expect("Unable to build script");

        let api = ObjectBuilder::default()
            .name("orchestra-api".to_string())
            .docker_container_builder(container)
            .script(script)
            .volatile_docker_data(config)
            .volatile_docker_data(shell)
            .docker_data(docker_data)
            .build()
            .expect("Unable to build object");

        // Inject consumer which consumes from kafka and sends it to the API
        let mut consumer = KafkaConsumer::default();
        consumer.args = args;

        let mut kafka_api_consumer_object = ExtObject::default();
        kafka_api_consumer_object.name = Some("kafka-api-consumer".to_string());

        let consumer = consumer.create(&kafka_api_consumer_object);

        let mut kafka_api_object = Object::default();

        kafka_api_object.name = "kafka-api".to_string();
        kafka_api_object.graph.ignore = true;

        let kafka = Kafka::new(vec!["orchestra-log".to_string()], self.kafka_host.host);
        kafka_api_object.service_config = Some(ServiceConfig::Kafka(kafka));

        kafka_api_object.node = Some(self.kafka_host.to_internal());

        (kafka_api_object, consumer)
    }
}
