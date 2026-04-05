use std::fmt::Display;
use serde::{Deserialize, Serialize};
use crate::types::preconfigured::sensor::Sensor;
use crate::object::Object;
use super::preconfigured::*;
use crate::traits::Configurable;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PreconfiguredType {
    PostgreSQL,
    Redis,
    MongoDB,
    Polypheny,

    Flink,
    Kafka,
    Spark,
    Storm,
}

impl PreconfiguredType {
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

    pub fn get_default(&self) -> PreconfiguredTypeConfig {
        match self {
            Self::PostgreSQL => PreconfiguredTypeConfig::PostgreSQL(PostgreSQL::default()),
            Self::Redis => PreconfiguredTypeConfig::Redis(Redis::default()),
            Self::MongoDB => PreconfiguredTypeConfig::MongoDB(MongoDB::default()),
            Self::Polypheny => PreconfiguredTypeConfig::Polypheny(Polypheny::default()),
            Self::Flink => PreconfiguredTypeConfig::Flink(Flink::new()),
            Self::Kafka => PreconfiguredTypeConfig::Kafka(Kafka::default()),
            Self::Spark => PreconfiguredTypeConfig::Spark(Spark::new()),
            Self::Storm => PreconfiguredTypeConfig::Storm(Storm::new()),
        }
    }
}

impl Display for PreconfiguredType {
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
pub enum PreconfiguredTypeConfig {
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

impl Configurable<Object> for PreconfiguredTypeConfig {
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
