use crate::{arguments::Arguments, deserialize_levelfilter, serialize_levelfilter};
use data_orchestra_core::{
    adapters::portainer::portainer::Portainer, interface::upload::UploadTypes,
    process::types::Kafka, shared::Amount,
};
use serde::{Deserialize, Serialize};
use tracing::level_filters::LevelFilter;

/// Contains metadata configuration for the API and its subsystems
#[derive(Debug, Deserialize, Serialize)]
pub struct MetaAPIConfig {
    pub api: APIConfig,
    pub portainer: Option<Portainer>,
    pub config: Option<Amount<String>>,
    pub ssh_key: Option<SshKeySharing>,
    #[serde(default)]
    pub uploader: Option<UploadTypes>,
}

/// Main configuration of the API instante of the orchestrator
#[derive(Debug, Deserialize, Serialize)]
pub struct APIConfig {
    #[serde(default = "APIConfig::default_ip")]
    pub ip: String,
    #[serde(deserialize_with = "deserialize_levelfilter")]
    #[serde(serialize_with = "serialize_levelfilter")]
    pub log_level: LevelFilter,
    #[serde(default = "APIConfig::default_port")]
    pub port: u16,
    #[serde(default)]
    pub remote_logger: Option<Kafka>,
}

impl APIConfig {
    fn default_ip() -> String {
        "127.0.0.1".to_string()
    }

    fn default_port() -> u16 {
        5000
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SshKeySharing {
    All(SshKey),
    Individual(Amount<SshKey>),
}

/// Ssh Key specification for a remote node connection
#[derive(Debug, Deserialize, Serialize)]
pub struct SshKey {
    // Path of ssh key
    pub path: String,
}

impl MetaAPIConfig {
    /// Combine CLI arguments with the configuration file.
    /// Provided CLI arguments overwrite the fields in the configuration
    pub fn combine(&mut self, args: Arguments) {
        self.api.log_level = args.log_level;
        if let Some(file) = args.file {
            let configs = self.config.get_or_insert(Amount::None);
            configs.insert(file);
        }
    }
}
