use log::LevelFilter;
use serde::{de::Error, Deserialize, Deserializer};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct Arguments {
    pub api_port: u16,

    pub topics: Option<String>,

    pub kafka_address: String,

    #[serde(deserialize_with = "deserialize_levelfilter")]
    pub level: LevelFilter,

    pub vec_topics: Vec<String>
}

pub fn deserialize_levelfilter<'de, D>(deserializer: D) -> Result<LevelFilter, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    let first = value.to_string();

    match first.to_lowercase().trim() {
        "error" => Ok(LevelFilter::Error),
        "warn" => Ok(LevelFilter::Warn),
        "info" => Ok(LevelFilter::Info),
        "debug" => Ok(LevelFilter::Debug),
        "trace" => Ok(LevelFilter::Trace),
        "off" => Ok(LevelFilter::Off),
        _ => Err(Error::custom("No Value exists"))
    }

}
