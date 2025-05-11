use serde::{Deserialize, Serialize};

use crate::core::types::Data;

use super::types::sensor::Sensor;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GeneratorType {
    Sensor
}

impl GeneratorType {
    pub fn new(&self) -> GeneratorTypeConfig {
        match self {
            Self::Sensor => GeneratorTypeConfig::Sensor(Sensor::new())
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GeneratorTypeConfig {
    Sensor(Sensor)
}

impl GeneratorTypeConfig {
    pub fn create(&self) -> Data {
        match self {
            Self::Sensor(ref sensor) => sensor.create()
        }
    }
}
