use log::LevelFilter;
use serde::{de::Error, Deserialize, Deserializer};

#[derive(Debug, Clone, Deserialize)]
pub struct Arguments {
    /// Address where data should be send to
    pub address: String,    
    /// Address (host:port) of kafka 
    pub consumer: String,
    /// Group id of consumer
    pub group_id: String,
    /// Kafka topics consumer should consume from
    pub topics: Vec<String>,
    /// Logging level
    #[serde(deserialize_with = "deserialize_levelfilter")]
    pub level: LevelFilter
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
