use crate::core::object::Object;
use crate::core::traits::Configurator;
use crate::core::types::services::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Service {
    PostgreSQL,
    Redis,
    MongoDB,
    Polypheny,

    Flink,
    Kafka,
    Spark,
    Storm,
}

impl Service {
    pub fn has_default(&self) -> bool {
        match self {
            Self::PostgreSQL => true,
            Self::Redis => true,
            Self::MongoDB => true,
            Self::Polypheny => true,
            Self::Flink => true,
            Self::Kafka => true,
            Self::Spark => true,
            Self::Storm => true,
        }
    }

    pub fn get_default(&self) -> ServiceConfig {
        match self {
            Self::PostgreSQL => ServiceConfig::PostgreSQL(PostgreSQL::default()),
            Self::Redis => ServiceConfig::Redis(Redis::default()),
            Self::MongoDB => ServiceConfig::MongoDB(MongoDB::default()),
            Self::Polypheny => ServiceConfig::Polypheny(Polypheny::default()),
            Self::Flink => ServiceConfig::Flink(Flink::new()),
            Self::Kafka => ServiceConfig::Kafka(Kafka::default()),
            Self::Spark => ServiceConfig::Spark(Spark::new()),
            Self::Storm => ServiceConfig::Storm(Storm::new()),
        }
    }
}

impl Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::PostgreSQL => String::from("PostgreSQL"),
            Self::Redis => String::from("Redis"),
            Self::MongoDB => String::from("MongoDB"),
            Self::Polypheny => String::from("Polypheny"),
            Self::Flink => String::from("Flink"),
            Self::Kafka => String::from("Kafka"),
            Self::Spark => String::from("Spark"),
            Self::Storm => String::from("Storm"),
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
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
