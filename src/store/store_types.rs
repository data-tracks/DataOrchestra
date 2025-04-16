use serde::{Deserialize, Serialize};
use crate::{docker::docker_struct::Container, types::amount::Amount};

use super::types::{mongodb::MongoDB, polypheny::Polypheny, postgres::PostGres, redis::Redis};

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
#[serde(untagged)]
pub enum StoreTypeConfig {
    PostGres(PostGres),
    Redis(Redis),
    MongoDB(MongoDB),
    Polypheny(Polypheny)
}

impl StoreType {
    pub fn new(&self) -> StoreTypeConfig {
        match self {
            StoreType::PostGres => StoreTypeConfig::PostGres(PostGres::new()),
            StoreType::Redis => StoreTypeConfig::Redis(Redis::new()),
            StoreType::MongoDB => StoreTypeConfig::MongoDB(MongoDB::new()),
            StoreType::Polypheny => StoreTypeConfig::Polypheny(Polypheny::new())
        } 
    }
}

impl StoreTypeConfig {
    /// Setup the given docker container with config of specified [`StoreType`]
    /// Consumes the docker container object and returns the modified container
    pub fn setup_container(&self, docker: Container) -> Container {
        match self {
            StoreTypeConfig::PostGres(postgres) => postgres.setup_container(docker),
            StoreTypeConfig::Redis(redis) => redis.setup_container(docker),
            StoreTypeConfig::MongoDB(mongodb) => mongodb.setup_container(docker),
            StoreTypeConfig::Polypheny(polypheny) => polypheny.setup_container(docker)
        }
    }

    pub fn mount_data(&self, data: Amount<String>, docker: Container) -> Container {
        match self {
            StoreTypeConfig::PostGres(postgres) => postgres.mount_data(data, docker),
            StoreTypeConfig::Redis(redis) => redis.mount_data(data ,docker),
            StoreTypeConfig::MongoDB(mongodb) => mongodb.mount_data(data, docker),
            StoreTypeConfig::Polypheny(polypheny) => polypheny.mount_data(data, docker)
        }
    }
}

