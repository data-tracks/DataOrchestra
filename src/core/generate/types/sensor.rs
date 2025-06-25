use std::collections::HashMap;

use log::LevelFilter;
use serde::{Deserialize, Serialize};

use crate::core::{generate::Generate, process::process_types::ProcessType, traits::Configurator, types::data::{DockerDataBuilder, VolatileDockerDataBuilder}};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub fn parse(&self) -> String {
        let mut command = String::from("cargo run --");


        if let Some(ref process) = self.stream_processor {
            command = format!("{command} --stream-processor {}", process.to_string().to_lowercase());
        }

        command = format!("{command} --interval {}", self.interval);

        if !self.topics.is_empty() {
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

impl Configurator<Generate> for Sensor {
    fn configure(&mut self, parent: &mut Generate) {
        let docker_data = DockerDataBuilder::default()
            .path("templates/generators/sensor")
            .destination("/sensor")
            .start(format!("/sensor/start.sh {}", self.parse()))
            .build()
            .expect("Unable to build docker sensor data");

        let json = serde_json::to_string_pretty(self).expect("Unable to parse sensor to string");

        let volatile_data = VolatileDockerDataBuilder::default()
            .file("/sensor/config.json")
            .data(json)
            .build()
            .expect("Unable to build volatile sensor data");

        let docker = parent.object.docker_container_builder.get_or_insert_default();

        docker
            .try_name("sensor")
            .dockerfile("images/rust.dockerfile")
            .image("rust_base");
    
        parent.object.volatile_docker_data.push(volatile_data);
        parent.object.docker_data.push(docker_data);
    }
}
