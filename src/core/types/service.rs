use crate::core::object::Object;
use crate::core::traits::Configurator;
use crate::core::types::services::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum ServiceConfig {
    // Stores
    PostgreSQL(PostgreSQL),
    Redis(Redis),
    MongoDB(MongoDB),
    Polypheny(Polypheny),

    // Streams / Processing
    Flink(Flink),
    Kafka(Kafka),
    Spark(Spark),
    Storm(Storm),

    // Generators
    Sensor(Sensor),
}

impl Configurator<Object> for ServiceConfig {
    fn configure(&mut self, parent: &mut Object) {
        match self {
            Self::PostgreSQL(postgres) => postgres.configure(parent),
            Self::Redis(redis) => redis.configure(parent),
            Self::MongoDB(mongodb) => mongodb.configure(parent),
            Self::Polypheny(polypheny) => polypheny.configure(parent),

            Self::Kafka(kafka) => kafka.configure(parent),
            Self::Flink(_flink) => panic!("Flink not implemented yet"),
            Self::Spark(_spark) => panic!("Spark implemented yet"),
            Self::Storm(_storm) => panic!("Storm not implemented yet"),

            Self::Sensor(sensor) => sensor.configure(parent),
        }
    }
}
