use serde::{Deserialize, Serialize};

use crate::core::traits::Configurator;

use super::{types::sensor::Sensor, Generate};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GeneratorType {
    Sensor
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GeneratorTypeConfig {
    Sensor(Sensor)
}

impl Configurator<Generate> for GeneratorTypeConfig {
    fn configure(&mut self, parent: &mut Generate) {
        match self {
            GeneratorTypeConfig::Sensor(sensor) => sensor.configure(parent),
        }
    }
}
