use crate::{arguments::Arguments, deserialize_levelfilter, serialize_levelfilter};
use data_orchestra_core::{
    adapters::Portainer, interface::upload::UploadTypes, process::types::Kafka, shared::Amount,
};
use serde::{Deserialize, Serialize};
use tracing::level_filters::LevelFilter;

/// Contains metadata configuration for the API and its subsystems
#[derive(Debug, Deserialize, Serialize)]
pub struct MetaAPIConfig {
    pub api: APIConfig,
    pub portainer: Option<Portainer>,
    pub config: Option<Amount<String>>,
    pub ssh_key: Option<Amount<SshKey>>,
    #[serde(default)]
    pub uploader: Option<UploadTypes>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct APIConfig {
    #[serde(default = "APIConfig::default_ip")]
    pub ip: String,
    #[serde(deserialize_with = "deserialize_levelfilter")]
    #[serde(serialize_with = "serialize_levelfilter")]
    pub log_level: LevelFilter,
    pub port: u16,
    #[serde(default)]
    pub remote_logger: Option<Kafka>,
}

impl APIConfig {
    fn default_ip() -> String {
        "127.0.0.1".to_string()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SshKey {
    #[serde(default)]
    pub all: bool,
    // Name of node which ssh key belongs to
    #[serde(default)]
    pub name: Option<Amount<String>>,
    // Path of ssh key
    pub path: String,
}

impl MetaAPIConfig {
    pub fn combine(&mut self, args: Arguments) {
        self.api.log_level = args.log_level
    }
}
