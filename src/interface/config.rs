use std::collections::HashMap;

use log::debug;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use crate::core::adapters::portainer;
use crate::core::adapters::portainer::portainer::Portainer;
use crate::core::process::process_types::{ProcessType, ProcessTypeConfig};
use crate::core::process::types::Kafka;
use crate::shared::{Amount, File};
use super::general::General;
use super::store::ExtStore;
use super::process::ExtProcess;
use super::object::ExtObject;
use super::generate::ExtGenerate;
use super::docker::ExtDocker;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    #[serde(default)]
    pub portainer: Portainer,
    #[serde(default)]
    pub variables: HashMap<String, String>,
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

    Err(serde::de::Error::custom(
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

    Err(serde::de::Error::custom(
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
 

impl Default for Config {
    fn default() -> Self {
        let mut hashmap= HashMap::<String, String>::new();
        hashmap.insert("key".to_string(), "value".to_string());
        Config 
        { 
            portainer: Portainer 
            {
                volume: portainer::default_volume(),
                host: portainer::default_host(),
                port: portainer::default_port(),
                username: portainer::default_username(),
                password: portainer::default_password(),
                jwt: "".to_string(),
                runner: portainer::default_runner()
            },
            variables: HashMap::new(),
            generate: Amount::Single
                (
                    ExtGenerate 
                    {
                        generate_type: None,
                        config: None,
                        amount: 1,
                        general: General 
                        {
                            docker: Some
                                (
                                    ExtDocker
                                    {
                                        name: Some("".to_string()),
                                        network: Some("".to_string()),
                                        enviroment: Some(hashmap.clone()),
                                        mount: Amount::None,
                                        publish_all: true,
                                        image: Some("".to_string()),
                                        dockerfile: Some("".to_string()),
                                        build_args: Some(hashmap.clone()),
                                        compose: Some("".to_string())
                                    }
                                ),
                            node: None,
                            file: Amount::Single
                            (
                                File 
                                { 
                                    name: None, 
                                    path: None, 
                                    destination: None, 
                                    start: None,
                                    dependency: None
                                }
                            ),
                            ansible: Some(String::from("scripts/ansible/ansible-setup.yaml"))
                        }
                    }
                 ), 
            process: Amount::Single
                (
                    ExtProcess
                    {
                        process_type: Some(ProcessType::Kafka),
                        config: Some(ProcessTypeConfig::Kafka(Kafka::new())),
                        amount: 1, 
                        general: General 
                        {
                            docker: Some
                                (
                                    ExtDocker
                                    {
                                        name: Some("".to_string()),
                                        network: Some("".to_string()),
                                        enviroment: Some(hashmap.clone()),
                                        mount: Amount::None,
                                        publish_all: true,
                                        image: Some("".to_string()),
                                        dockerfile: Some("".to_string()),
                                        build_args: Some(hashmap.clone()),
                                        compose: Some("".to_string())
                                    }
                                ),
                            node: None,
                            file: Amount::Single
                            (
                                File 
                                { 
                                    name: None, 
                                    path: None, 
                                    destination: None, 
                                    start: None,
                                    dependency: None
                                }
                            ),
                            ansible: Some(String::from("scripts/ansible/ansible-setup.yaml"))
                        }
                    }
                ), 
            store: Amount::Single
                (
                    ExtStore 
                    { 
                        general: General 
                        {
                            docker: Some
                                (
                                    ExtDocker
                                    {
                                        name: Some("".to_string()),
                                        network: Some("".to_string()),
                                        enviroment: Some(hashmap.clone()),
                                        mount: Amount::None,
                                        publish_all: true,
                                        image: Some("".to_string()),
                                        dockerfile: Some("".to_string()),
                                        build_args: Some(hashmap.clone()),
                                        compose: Some("".to_string())
                                    }
                                ),
                            node: None,
                            file: Amount::Single
                            (
                                File 
                                { 
                                    name: None, 
                                    path: None, 
                                    destination: None, 
                                    start: None,
                                    dependency: None
                                }
                            ),
                            ansible: Some(String::from("scripts/ansible/ansible-setup.yaml"))
                        },
                        schema: Amount::Single("".to_string()),
                        db_type: None,
                        config: None
                    }
                ), 
            object: Amount::Single
                (
                    ExtObject 
                    { 
                        amount: 1, 
                        general: General 
                        {
                            docker: Some
                                (
                                    ExtDocker
                                    {
                                        name: Some("".to_string()),
                                        network: Some("".to_string()),
                                        enviroment: Some(hashmap.clone()),
                                        mount: Amount::None,
                                        publish_all: true,
                                        image: Some("".to_string()),
                                        dockerfile: Some("".to_string()),
                                        build_args: Some(hashmap.clone()),
                                        compose: Some("".to_string())
                                    }
                                ),
                            node: None,
                            file: Amount::Single
                            (
                                File 
                                { 
                                    name: None, 
                                    path: None, 
                                    destination: None, 
                                    start: None,
                                    dependency: None
                                }
                            ),
                            ansible: Some(String::from("scripts/ansible/ansible-setup.yaml"))
                        },  
                    }
                )  
        }
    }
}
