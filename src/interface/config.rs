use log::debug;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::Error;
use serde_json::Value;
use crate::core::adapters::portainer::portainer::Portainer;
use crate::shared::Amount;
use super::store::ExtStore;
use super::process::ExtProcess;
use super::object::ExtObject;
use super::generate::ExtGenerate;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    #[serde(default)]
    pub portainer: Portainer,

    #[serde(default)]
    #[serde(deserialize_with = "deserialize_generate")]
    pub generate: Amount<ExtGenerate>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_process")]
    pub process: Amount<ExtProcess>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_store")]
    pub store: Amount<ExtStore>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_object")]
    pub object: Amount<ExtObject>,
}

pub fn deserialize_generate<'de, D>(deserializer: D) -> Result<Amount<ExtGenerate>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    let first = value.to_string();

    if first.chars().next().is_some_and(|x| x == '[') {
        let result = Vec::deserialize(value.clone());
        if let Ok(compact) = result {
            return Ok(Amount::Multiple(compact));
        }    
        else if let Err(error) = result {
            debug!("{}", value.clone());
            panic!("Error while deserializing Vec<generate> [{}]", error);
        }
    }
    else {
        let result = ExtGenerate::deserialize(value.clone());
        if let Ok(full) = result {
            return Ok(Amount::Single(full));
        }
        else if let Err(error) = result {
            debug!("{}", value.clone());
            panic!("Error while deserializing generate: [{}]", error);
        }  
    }

    Err(Error::custom(
        "Could not deserialize into either a single store or a list of them",
    ))
}

pub fn deserialize_process<'de, D>(deserializer: D) -> Result<Amount<ExtProcess>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    let first = value.to_string();

    if first.chars().next().is_some_and(|x| x == '[') {
        let result = Vec::deserialize(value.clone());
        if let Ok(compact) = result {
            return Ok(Amount::Multiple(compact));
        }    
        else if let Err(error) = result {
            debug!("{}", value.clone());
            panic!("Error while deserializing Vec<process> [{}]", error);
        }
    }
    else {
        let result = ExtProcess::deserialize(value.clone());
        if let Ok(full) = result {
            return Ok(Amount::Single(full));
        }
        else if let Err(error) = result {
            debug!("{}", value.clone());
            panic!("Error while deserializing process: [{}]", error);
        }  
    }

    Err(Error::custom(
        "Could not deserialize into either a single process or a list of them",
    ))
}

pub fn deserialize_store<'de, D>(deserializer: D) -> Result<Amount<ExtStore>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    let first = value.to_string();

    if first.chars().next().is_some_and(|x| x == '[') {
        let result = Vec::deserialize(value.clone());
        if let Ok(compact) = result {
            return Ok(Amount::Multiple(compact));
        }    
        else if let Err(error) = result {
            debug!("{}", value.clone());
            panic!("Error while deserializing Vec<store> [{}]", error);
        }
    }
    else {
        let result = ExtStore::deserialize(value.clone());
        if let Ok(full) = result {
            return Ok(Amount::Single(full));
        }
        else if let Err(error) = result {
            debug!("{}", value.clone());
            panic!("Error while deserializing store: [{}]", error);
        }  
    }

    Err(serde::de::Error::custom(
        "Could not deserialize into either a single store or a list of them",
    ))
}

pub fn deserialize_object<'de, D>(deserializer: D) -> Result<Amount<ExtObject>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    let first = value.to_string();

    if first.chars().next().is_some_and(|x| x == '[') {
        let result = Vec::deserialize(value.clone());
        if let Ok(compact) = result {
            return Ok(Amount::Multiple(compact));
        }    
        else if let Err(error) = result {
            debug!("{}", value.clone());
            panic!("Error while deserializing Vec<object> [{}]", error);
        }
    }
    else {
        let result = ExtObject::deserialize(value.clone());
        if let Ok(full) = result {
            return Ok(Amount::Single(full));
        }
        else if let Err(error) = result {
            debug!("{}", value.clone());
            panic!("Error while deserializing object: [{}]", error);
        }  
    }

    Err(serde::de::Error::custom(
        "Could not deserialize into either a single object or a list of them",
    ))
}
