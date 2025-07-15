use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::core::adapters::{ContainerBuilder, TmuxBuilder};
use crate::core::types::ScriptBuilder;
use crate::core::types::data::{DataBuilder, VolatileDataBuilder};
use crate::core::traits::Creator;
use crate::core::process::{process_types::ProcessTypeConfig, types::Kafka, Process};
use crate::core::object::{Object, ObjectBuilder};
use crate::core::attach::types::kafka_consumer::KafkaConsumer;
use crate::core::attach::types::kafka_consumer::ArgumentsBuilder;
use crate::shared::ToInternal;
use super::{general::General, node::ExtNode};

#[derive(Debug, Deserialize, Serialize)]
pub struct API {
    #[serde(default = "default_port")]
    port: u16,
    kafka_host: ExtNode 
}

pub fn default_port() -> u16 {
    5000
}

impl ToInternal<(Process, Object, Object, u16)> for API {
    fn to_internal(self) -> (Process, Object, Object, u16) {
        let docker_name = "orchestra-api".to_string();

        // Build the api object
        let args = ArgumentsBuilder::default()
            .address(format!("http://host.docker.internal:{}/orchestra/broadcast", self.port))
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

        let mut general = General::default(); 
        general.name = Some("kafka-api-consumer".to_string());

        let consumer = consumer.create(&general);

        let mut process = Process::default();

        process.object.name = "kafka-api".to_string();
        process.object.graph.ignore = true;

        let kafka = Kafka::new(vec!["orchestra-log".to_string()], self.kafka_host.host);
        process.config = Some(ProcessTypeConfig::Kafka(kafka));

        process.object.node = Some(self.kafka_host.to_internal());

        (process, consumer, api, self.port)
    } 
}
