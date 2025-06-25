use serde::{Deserialize, Serialize};
use crate::core::traits::Configurator;
use super::{types::{MongoDB, Polypheny, PostGres, Redis}, Store};

/// Store Type. Represents available storage types 
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StoreType {
    PostGres,
    Redis,
    MongoDB,
    Polypheny
}

/// Represents the available storing types
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StoreTypeConfig {
    PostGres(PostGres),
    Redis(Redis),
    MongoDB(MongoDB),
    Polypheny(Polypheny)
}

impl StoreType {
    pub fn get_default(&self) -> StoreTypeConfig {
        match self {
            StoreType::PostGres => StoreTypeConfig::PostGres(PostGres::default()),
            StoreType::Redis => StoreTypeConfig::Redis(Redis::default()),
            StoreType::MongoDB => StoreTypeConfig::MongoDB(MongoDB::default()),
            StoreType::Polypheny => StoreTypeConfig::Polypheny(Polypheny::default())
        } 
    }
}

impl Configurator<Store> for StoreTypeConfig {
    fn configure(&mut self, parent: &mut Store) {
        match self {
            StoreTypeConfig::PostGres(postgres) => postgres.configure(parent),
            StoreTypeConfig::Redis(redis) => redis.configure(parent),
            StoreTypeConfig::MongoDB(mongodb) => mongodb.configure(parent),
            StoreTypeConfig::Polypheny(polypheny) => polypheny.configure(parent),
        };
    }
}
