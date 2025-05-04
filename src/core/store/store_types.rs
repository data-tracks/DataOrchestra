use serde::{Deserialize, Serialize};
use crate::core::adapters::docker::container::ContainerBuilder;
use super::types::{MongoDB, Polypheny, PostGres, Redis};

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
/// 
/// # Json
///
/// Tied to the [`StoreType`] field.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
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
    pub fn setup_container(&self, docker: &mut ContainerBuilder) {
        match self {
            StoreTypeConfig::PostGres(postgres) => postgres.setup_container(docker),
            StoreTypeConfig::Redis(redis) => redis.setup_container(docker),
            StoreTypeConfig::MongoDB(mongodb) => mongodb.setup_container(docker),
            StoreTypeConfig::Polypheny(polypheny) => polypheny.setup_container(docker),
        };
    }

    pub fn mount_data(&self, data: &Vec<String>, docker: &mut ContainerBuilder) {
        match self {
            StoreTypeConfig::PostGres(postgres) => postgres.mount_data(data, docker),
            StoreTypeConfig::Redis(_redis) => panic!("No mount data for redis"),
            StoreTypeConfig::MongoDB(_mongodb) => panic!("No mount data for mongodb"),
            StoreTypeConfig::Polypheny(_polypheny) => panic!("No mount data for polypheny")
        };
    }
}
