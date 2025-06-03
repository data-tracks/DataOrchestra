use std::collections::HashMap;

use log::LevelFilter;
use serde::{Deserialize, Serialize};

use crate::core::{process::process_types::ProcessType, types::DockerData};

#[derive(Debug, Serialize, Deserialize)]
pub struct Sensor {
    #[serde(default)]
    pub stream_processor: Option<ProcessType>,
    #[serde(default = "default_interval")]
    pub interval: u64,
    #[serde(default = "default_address")]
    pub address: String,
    #[serde(default)]
    pub topics: Vec<String>,
    additional: Option<HashMap<String, String>>,
}

pub fn default_interval() -> u64 {
    1
}

pub fn default_address() -> String {
    String::from("localhost:9092")
}

pub fn default_level() -> LevelFilter {
    LevelFilter::Info
}

impl Sensor {
    pub fn new() -> Self {
        Sensor { 
            stream_processor: None,
            interval: 1,
            address: String::from("localhost:9092"),
            topics: Vec::new(),
            additional: Some(HashMap::new())
        }
    }

    pub fn create(&self) -> DockerData {
        DockerData {
            name: String::new(),
            path: String::from("templates/generators/sensor"),
            destination: String::from("/sensor"),
            env: None,
            start: format!("cd /sensor && sh spawn.sh \"Sensor\" \"{}\"", self.parse()),
            dependency: None
        }
    }

    pub fn parse(&self) -> String {
        let mut command = String::from("cargo run --");


        if let Some(ref process) = self.stream_processor {
            command = format!("{command} --stream-processor {}", process.to_string().to_lowercase());
        }

        command = format!("{command} --interval {}", self.interval);

        if self.topics.len() > 0 {
            for topic in self.topics.iter() {
                command = format!("{command} --topic {}", topic);
            }
        }

        command = format!("{command} --address {}", self.address);

        if let Some(ref options) = self.additional {
            for (key, value) in options.iter() {
                command = format!("{command} -{key} {value}");
            }
        }

        command
    }
}
