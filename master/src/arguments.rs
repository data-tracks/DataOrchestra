use crate::{deserialize_levelfilter, serialize_levelfilter};
use clap::Parser;
use serde::{Deserialize, Serialize};
use tracing_subscriber::filter::LevelFilter;

/// CLI arguments for the Orchestrator
#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
#[command(version, about)]
pub struct Arguments {
    /// Config file location
    #[arg(long)]
    #[serde(default = "Arguments::default_config_file")]
    pub config_file: String,

    /// Distributed components file
    #[arg(short , long)]
    #[serde(default)]
    pub components_file: Option<String>,

    /// Logging level
    #[arg(short, long, default_value_t = LevelFilter::INFO)]
    #[serde(default = "Arguments::default_level")]
    #[serde(deserialize_with = "deserialize_levelfilter")]
    #[serde(serialize_with = "serialize_levelfilter")]
    pub log_level: LevelFilter,

    /// Remove all running and stopped docker containers as well as all networks
    #[arg(long = "remove_all", default_value_t = false)]
    #[serde(default)]
    pub remove_all: bool,

    /// Setup portainer manager
    #[arg(long = "portainer", default_value_t = true)]
    #[serde(default = "Arguments::default_portainer")]
    pub portainer: bool,

    /// Valid private ssh key for validating remote node connection
    #[arg(short, long)]
    #[serde(default)]
    pub ssh_key: Option<String>,
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments {
            config_file: "./config.toml".to_string(),
            components_file: None,
            log_level: LevelFilter::INFO,
            remove_all: false,
            portainer: true,
            ssh_key: None,
        }
    }
}

impl Arguments {
    pub fn default_config_file() -> String {
        "config.toml".to_string()
    }

    pub fn default_portainer() -> bool {
        true
    }

    pub fn default_level() -> LevelFilter {
        LevelFilter::INFO
    }
}
