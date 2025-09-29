use log::LevelFilter;
use serde::{Deserialize, Serialize};

use crate::adapters::TmuxBuilder;
use crate::{
    generate::Generate,
    traits::Configurator,
    types::{
        Executables, ScriptBuilder,
        data::{DataBuilder, DataTypes, VolatileDataBuilder},
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sensor {
    #[serde(default = "default_interval")]
    pub interval: u64,
    pub address: String,
}

pub fn default_interval() -> u64 {
    1
}

pub fn default_level() -> LevelFilter {
    LevelFilter::Info
}

impl Sensor {
    pub fn parse(&self) -> String {
        let mut command = String::from("cargo run --");

        command = format!("{command} --interval {}", self.interval);

        command = format!("{command} --address {}", self.address);

        command
    }
}

impl Configurator<Generate> for Sensor {
    fn configure(&mut self, parent: &mut Generate) {
        let data = DataBuilder::default()
            .src("services/sensor")
            .dst("/sensor")
            .build()
            .expect("Unable to build docker sensor data");

        let tmux = TmuxBuilder::default()
            .session("Sensor")
            .command(self.parse())
            .build();

        let volatile = VolatileDataBuilder::default()
            .dst("/sensor/start.sh")
            .content(tmux)
            .build()
            .expect("Unable to build volatile sensor script");

        let script = ScriptBuilder::default()
            .path("/sensor/start.sh")
            .build()
            .expect("Unable to build script");

        let docker = parent
            .object
            .docker_container_builder
            .get_or_insert_default();

        docker
            .try_name("sensor")
            .dockerfile("images/rust.dockerfile")
            .image("rust_base");

        parent
            .object
            .resources
            .push(DataTypes::VolatileData(volatile));
        parent.object.resources.push(DataTypes::Data(data));
        parent.object.executables.push(Executables::Script(script));
    }
}
