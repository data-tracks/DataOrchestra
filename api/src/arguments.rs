use clap::Parser;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};
use tracing_subscriber::filter::LevelFilter;

/// CLI arguments for the Orchestrator
#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
#[command(version, about)]
pub struct Arguments {
    /// Config file location
    #[arg(short, long)]
    #[arg(env = "FILE")]
    #[serde(default)]
    pub file: Option<String>,

    /// Logging level
    #[arg(short, long, default_value_t = LevelFilter::INFO)]
    #[arg(env = "LEVEL")]
    #[serde(default = "Arguments::default_level")]
    #[serde(deserialize_with = "deserialize_levelfilter")]
    #[serde(serialize_with = "serialize_levelfilter")]
    pub level: LevelFilter,

    /// Remove all running and stopped docker containers as well as all networks
    #[arg(long = "remove_all", default_value_t = false)]
    #[arg(env = "REMOVE_ALL")]
    #[serde(default)]
    pub remove_all: bool,

    /// Setup portainer manager
    #[arg(long = "portainer", default_value_t = true)]
    #[arg(env = "PORTAINER")]
    #[serde(default = "Arguments::default_portainer")]
    pub portainer: bool,

    /// Valid private ssh key for validating remote node connection
    #[arg(short, long)]
    #[arg(env = "SSH_KEY")]
    #[serde(default)]
    pub ssh_key: Option<String>,

    /// Port of the API
    #[arg(short, long)]
    #[arg(env = "API_PORT")]
    #[serde(default = "Arguments::default_api_port")]
    pub api_port: u16,
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments {
            file: None,
            level: LevelFilter::INFO,
            remove_all: false,
            portainer: true,
            ssh_key: None,
            api_port: Arguments::default_api_port(),
        }
    }
}

impl Arguments {
    pub fn default_portainer() -> bool {
        true
    }

    pub fn default_api_port() -> u16 {
        5000
    }

    pub fn default_level() -> LevelFilter {
        LevelFilter::INFO
    }
}

pub fn deserialize_levelfilter<'de, D>(deserializer: D) -> Result<LevelFilter, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    match s.to_lowercase().trim() {
        "error" => Ok(LevelFilter::ERROR),
        "warn" => Ok(LevelFilter::WARN),
        "info" => Ok(LevelFilter::INFO),
        "debug" => Ok(LevelFilter::DEBUG),
        "trace" => Ok(LevelFilter::TRACE),
        "off" => Ok(LevelFilter::OFF),
        _ => Err(Error::custom("No Value exists")),
    }
}

pub fn serialize_levelfilter<S>(level: &LevelFilter, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let string = match level {
        &LevelFilter::ERROR => "error",
        &LevelFilter::WARN => "warn",
        &LevelFilter::INFO => "info",
        &LevelFilter::DEBUG => "debug",
        &LevelFilter::TRACE => "trace",
        &LevelFilter::OFF => "off",
    };

    s.serialize_str(string)
}
