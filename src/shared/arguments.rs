use clap::Parser;
use log::LevelFilter;

use super::ObjectTypes;

/// CLI arguments for the Orchestrator
#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct Arguments {
    /// Config file location
    #[arg(short, long)]
    #[arg(env = "FILE")]
    pub file: Option<String>,

    /// Logging level
    #[arg(short, long, default_value_t = LevelFilter::Info)]
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

    /// Skip the portainer manager setup
    #[arg(long = "no_portainer", default_value_t = true)]
    #[arg(env = "NO_PORTAINER")]
    pub portainer: bool,

    /// Valid private ssh key for validating remote node connection
    #[arg(short, long)]
    #[arg(env = "SSH_KEY")]
    pub ssh_key: Option<String>,

    #[arg(short, long, default_value_t = false)]
    #[arg(env = "API_ONLY")]
    pub api_only: bool
}
