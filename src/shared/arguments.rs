use std::sync::OnceLock;

use clap::Parser;
use log::LevelFilter;

pub static ARGS: OnceLock<Arguments> = OnceLock::new();

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Arguments {
    /// Config file location
    #[arg(short, long)]
    pub file: Option<String>,

    /// Logging level
    #[arg(short, long, default_value_t = LevelFilter::Info)]
    pub level: LevelFilter,

    /// Remove all running and stopped docker containers aswell as all networks
    #[arg(long = "remove_all", default_value_t = false)]
    pub remove_all: bool,

    /// Generate a valid config file 
    #[arg(long = "generate_valid_json", default_value_t = false)]
    pub generate_valid_json: bool,

    /// Skip the portainer manager setup
    #[arg(long = "no_portainer", default_value_t = false)]
    pub no_portainer: bool,

    /// Valid private ssh key for validating remote node connection
    #[arg(short, long)]
    pub ssh_key: Option<String> 
}