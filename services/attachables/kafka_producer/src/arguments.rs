use log::LevelFilter;
use serde::{de::Error, Deserialize, Deserializer};

#[derive(Debug, Clone, Deserialize)]
pub struct Arguments {
    // Port of logging API
    pub api_port: u16,
    // Kafka address
    pub address: String,
    // Logging level
    #[serde(deserialize_with = "deserialize_levelfilter")]
    pub level: LevelFilter,
    // Kafka topics
    pub topics: Vec<String>,
    // Address of logging system
    #[serde(default)]
    pub logger: Option<String>
}

pub fn deserialize_levelfilter<'de, D>(deserializer: D) -> Result<LevelFilter, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    match s.to_lowercase().trim() {
        "error" => Ok(LevelFilter::Error),
        "warn" => Ok(LevelFilter::Warn),
        "info" => Ok(LevelFilter::Info),
        "debug" => Ok(LevelFilter::Debug),
        "trace" => Ok(LevelFilter::Trace),
        "off" => Ok(LevelFilter::Off),
        _ => Err(Error::custom("No Value exists"))
    }
}
