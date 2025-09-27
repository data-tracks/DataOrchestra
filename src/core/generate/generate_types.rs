use serde::{Deserialize, Serialize};

use crate::core::traits::Configurator;

use super::{Generate, types::sensor::Sensor};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratorTypeConfig {
    Sensor(Sensor),
}

impl Configurator<Generate> for GeneratorTypeConfig {
    fn configure(&mut self, parent: &mut Generate) {
        match self {
            GeneratorTypeConfig::Sensor(sensor) => sensor.configure(parent),
        }
    }
}
