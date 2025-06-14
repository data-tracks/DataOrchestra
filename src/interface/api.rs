use serde::{Deserialize, Serialize};

use crate::{core::process::{process_types::ProcessTypeConfig, types::Kafka, Process}, shared::ToInternal};

use super::node::ExtNode;

#[derive(Debug, Deserialize, Serialize)]
pub struct API {
    #[serde(default = "default_port")]
    port: u16,
    kafka_host: ExtNode 
}

pub fn default_port() -> u16 {
    5000
}

impl ToInternal<(Process, u16)> for API {
    fn to_internal(self) -> (Process, u16) {
        let mut process = Process::default();

        let kafka = Kafka::new(vec!["orchestra-log".to_string()], self.kafka_host.host);
        process.config = Some(ProcessTypeConfig::Kafka(kafka));

        process.object.node = Some(self.kafka_host.to_internal());

        (process, self.port)
    } 
}
