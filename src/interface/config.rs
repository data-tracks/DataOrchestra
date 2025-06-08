use log::debug;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::Error;
use serde_json::Value;
use crate::core::adapters::portainer::portainer::Portainer;
use crate::core::attach::attach_types::ToObject;
use crate::core::config::Config;
use crate::core::object::Object;
use crate::shared::{Amount, ToInternal, ToInternalVec};
use super::store::ExtStore;
use super::process::ExtProcess;
use super::object::ExtObject;
use super::generate::ExtGenerate;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtConfig {
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

impl ToInternal<(Config, Portainer)> for ExtConfig {
    fn to_internal(self) -> (Config, Portainer) {
        let config = Config 
        {
            generate: self.generate.to_internal(),
            process: self.process.to_internal(),
            store: self.store.to_internal(),
            object: self.object.to_internal()
        };
        let portainer = self.portainer;

        (config, portainer)
    }
}

impl ExtConfig {
    /// Extract the attachable components from the different components and parse them into their
    /// own objects
    pub fn extract_attachables(&mut self) -> Vec<Object> {
        let mut attach_objects = Vec::new();
        for object in self.object.to_mut_ref_vec() {
            for config in object.general.attach_config.take().to_vec() {
                attach_objects.push(config.to_object(&object.general));
            }
        }

        for store in self.store.to_mut_ref_vec() {
            for config in store.general.attach_config.take().to_vec() {
                attach_objects.push(config.to_object(&store.general));
            }
        } 

        for process in self.store.to_mut_ref_vec() {
            for config in process.general.attach_config.take().to_vec() {
                attach_objects.push(config.to_object(&process.general));
            }
        }
        
        for generate in self.generate.to_mut_ref_vec() {
            for config in generate.general.attach_config.take().to_vec() {
                attach_objects.push(config.to_object(&generate.general));
            }
        }

        attach_objects
    }
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
