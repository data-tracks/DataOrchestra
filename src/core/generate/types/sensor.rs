use serde::{Deserialize, Serialize};

use crate::core::{process::process_types::ProcessType, types::Data};

#[derive(Debug, Serialize, Deserialize)]
pub struct Sensor {
    #[serde(default)]
    pub stream_processor: Option<ProcessType>,
    #[serde(default = "default_interval")]
    pub interval: u64,
    #[serde(default = "default_address")]
    pub address: String,
    #[serde(default)]
    pub topics: Vec<String>
}

pub fn default_interval() -> u64 {
    1
}

pub fn default_address() -> String {
    String::from("localhost:9092")
}

impl Sensor {
    pub fn new() -> Self {
        Sensor { 
            stream_processor: None,
            interval: 1,
            address: String::from("localhost:9092"),
            topics: Vec::new()
        }
    }

    pub fn create(&self) -> Data {
        Data {
            name: String::new(),
            path: String::from("templates/generators/sensor"),
            destination: String::from("/"),
            start: format!("/sensor/{}", self.parse()),
            dependency: None
        }
    }

    pub fn parse(&self) -> String {
        let mut command = String::from("cargo run --");


        if let Some(ref process) = self.stream_processor {
            command = format!("{command} --stream_processor {}", process.to_string().to_lowercase());
        }

        command = format!("{command} --interval {}", self.interval);

        if self.topics.len() > 0 {
            for topic in self.topics.iter() {
                command = format!("{command} --topic {}", topic);
            }
        }

        command = format!("{command} --address {}", self.address);

        command
    }
}
