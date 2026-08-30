use clap::{Parser, ValueEnum};
use serde::Deserialize;
use tracing_subscriber::filter::LevelFilter;

use super::ObjectTypes;

#[derive(Debug, Deserialize, Clone, Copy, ValueEnum)]
pub enum APIUsage {
    Isolate,
    Enable,
    Disable,
}

impl APIUsage {
    pub fn is_isolate(&self) -> bool {
        match self {
            APIUsage::Isolate => true,
            _ => false,
        }
    }

    pub fn is_enabled(&self) -> bool {
        match self {
            APIUsage::Enable => true,
            _ => false,
        }
    }

    pub fn is_disabled(&self) -> bool {
        match self {
            APIUsage::Disable => true,
            _ => false,
        }
    }
}

impl Default for APIUsage {
    fn default() -> Self {
        APIUsage::Enable
    }
}

impl ToString for APIUsage {
    fn to_string(&self) -> String {
        let s = match self {
            APIUsage::Disable => "disable",
            APIUsage::Enable => "enable",
            APIUsage::Isolate => "isolate",
        };

        s.to_string()
    }
}

/// CLI arguments for the Orchestrator
#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct Arguments {
    /// Config file location
    #[arg(short, long)]
    #[arg(env = "FILE")]
    pub file: Option<String>,

    /// Logging level
    #[arg(short, long, default_value_t = LevelFilter::INFO)]
    #[arg(env = "LEVEL")]
    pub level: LevelFilter,

    /// Remove all running and stopped docker containers aswell as all networks
    #[arg(long = "remove_all", default_value_t = false)]
    #[arg(env = "REMOVE_ALL")]
    pub remove_all: bool,

    /// Generate a valid config file
    #[arg(long = "generate_valid_json")]
    #[arg(env = "GENERATE_VALID_JSON")]
    pub generate_valid_json: Option<ObjectTypes>,

    /// Setup portainer manager
    #[arg(long = "portainer", default_value_t = true)]
    #[arg(env = "PORTAINER")]
    pub portainer: bool,

    /// Valid private ssh key for validating remote node connection
    #[arg(short, long)]
    #[arg(env = "SSH_KEY")]
    pub ssh_key: Option<String>,

    #[arg(short, long, default_value_t = APIUsage::default())]
    #[arg(env = "API_USAGE")]
    pub api_usage: APIUsage,

    /// Start only subset if items provided by name from config
    #[arg(short, long)]
    #[arg(env = "ISOLATE")]
    pub isolate: Option<Vec<String>>,
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments {
            file: None,
            level: LevelFilter::INFO,
            remove_all: false,
            generate_valid_json: None,
            portainer: true,
            ssh_key: None,
            api_usage: APIUsage::default(),
            isolate: None,
        }
    }
}
