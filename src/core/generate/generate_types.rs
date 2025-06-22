use serde::{Deserialize, Serialize};

use crate::core::{object::Object, traits::Configurator};

use super::types::sensor::Sensor;


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

impl Configurator for GeneratorTypeConfig {
    fn configure(&mut self, object: &mut Object) {
        match self {
            GeneratorTypeConfig::Sensor(sensor) => sensor.configure(object),
        }
    }
}
