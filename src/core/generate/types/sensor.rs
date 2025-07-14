use std::collections::HashMap;

use log::LevelFilter;
use serde::{Deserialize, Serialize};

use crate::core::{generate::Generate, process::process_types::ProcessType, traits::Configurator, types::{data::{DataBuilder, DataTypes, VolatileDataBuilder}, Executables, ScriptBuilder}};

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
    //pub additional: Option<HashMap<String, String>>,
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
                command = format!("{command} --topic {topic}");
            }
        }

        command = format!("{command} --address {}", self.address);

        /*
        if let Some(ref options) = self.additional {
            for (key, value) in options.iter() {
                command = format!("{command} -{key} {value}");
            }
        }
        */

        command
    }
}

impl Configurator<Generate> for Sensor {
    fn configure(&mut self, parent: &mut Generate) {
        let docker_data = DataBuilder::default()
            .source("templates/generators/sensor")
            .destination("/sensor")
            //.start()
            .build()
            .expect("Unable to build docker sensor data");

        let script = ScriptBuilder::default()
            .path(format!("/sensor/start.sh {}", self.parse()))
            .build()
            .expect("Unable to build script");

        let json = serde_json::to_string_pretty(self).expect("Unable to parse sensor to string");

        let volatile_data = VolatileDataBuilder::default()
            .destination("/sensor/config.json")
            .content(json)
            .build()
            .expect("Unable to build volatile sensor data");
        
        let docker = parent.object.docker_container_builder.get_or_insert_default();

        docker
            .try_name("sensor")
            .dockerfile("images/rust.dockerfile")
            .image("rust_base");
    
        parent.object.resources.push(DataTypes::VolatileDockerData(volatile_data));
        parent.object.resources.push(DataTypes::DockerData(docker_data));
        parent.object.executables.push(Executables::Script(script));
    }
}
