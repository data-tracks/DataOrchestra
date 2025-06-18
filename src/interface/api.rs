use serde::{Deserialize, Serialize};

use crate::{core::{attach::{attach_types::ToObject, types::kafka_consumer::KafkaConsumer}, object::Object, process::{process_types::ProcessTypeConfig, types::Kafka, Process}}, shared::ToInternal};

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

impl ToInternal<(Process, Object, u16)> for API {
    fn to_internal(self) -> (Process, Object, u16) {
        // Inject consumer which consumes from kafka and sends it to the API
        let mut consumer = KafkaConsumer::default();
        consumer.args.topics.push("orchestra-log".to_string());
        consumer.args.group_id = "logger".to_string();
        consumer.args.consumer = format!("{}:9092", self.kafka_host.host.clone());
        consumer.args.address = format!("http://localhost:{}", self.port);

        let mut general = General::default(); 
        general.name = Some("kafka-api-consumer".to_string());

        let consumer = consumer.to_object(&general);

        let mut process = Process::default();

        process.object.name = "kafka-api".to_string();

        let kafka = Kafka::new(vec!["orchestra-log".to_string()], self.kafka_host.host);
        process.config = Some(ProcessTypeConfig::Kafka(kafka));

        process.object.node = Some(self.kafka_host.to_internal());

        (process, consumer, self.port)
    } 
}
