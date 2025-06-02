use std::sync::OnceLock;

use clap::Parser;
use log::LevelFilter;

pub static ARGS: OnceLock<Arguments> = OnceLock::new();

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Arguments {
    /// Config file location
    #[arg(short, long, env)]
    pub file: Option<String>,

    /// Logging level
    #[arg(short, long, env, default_value_t = LevelFilter::Info)]
    pub level: LevelFilter,

    /// Remove all running and stopped docker containers aswell as all networks
    #[arg(long = "remove_all", env, default_value_t = false)]
    pub remove_all: bool,

    /// Generate a valid config file 
    #[arg(long = "generate_valid_json", env, default_value_t = false)]
    pub generate_valid_json: bool,

    /// Skip the portainer manager setup
    #[arg(long = "no_portainer", env, default_value_t = false)]
    pub no_portainer: bool,

    /// Valid private ssh key for validating remote node connection
    #[arg(short, long, env)]
    pub ssh_key: Option<String> 
}
