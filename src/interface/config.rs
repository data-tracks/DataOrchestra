use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use crate::shared::{Amount, Node};
use super::store::ExtStore;
use super::process::ExtProcess;
use super::object::ExtObject;
use super::generate::ExtGenerate;
use super::docker::Docker;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub generate: Amount<ExtGenerate>,
    pub process: Amount<ExtProcess>,
    pub store: Amount<ExtStore>,
    pub object: Amount<ExtObject>,
}

impl Default for Config {
    fn default() -> Self {
        let mut hashmap= HashMap::<String, String>::new();
        hashmap.insert("key".to_string(), "value".to_string());
        Config 
        { 
            generate: Amount::Single
                (
                    ExtGenerate 
                    {
                        amount: 1,
                        general: General 
                        {
                            docker: Some
                                (
                                    Docker
                                    {
                                        name: Some("".to_string()),
                                        network: Some("".to_string()),
                                        options: Some(hashmap.clone()),
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
                                    start: None 
                                }
                            )
                        }
                    }
                 ), 
            process: Amount::Single
                (
                    ExtProcess 
                    { 
                        amount: 1, 
                        general: General 
                        {
                            docker: Some
                                (
                                    Docker
                                    {
                                        name: Some("".to_string()),
                                        network: Some("".to_string()),
                                        options: Some(hashmap.clone()),
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
                                    start: None 
                                }
                            )
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
                                    Docker
                                    {
                                        name: Some("".to_string()),
                                        network: Some("".to_string()),
                                        options: Some(hashmap.clone()),
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
                                    start: None 
                                }
                            )
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
                                    Docker
                                    {
                                        name: Some("".to_string()),
                                        network: Some("".to_string()),
                                        options: Some(hashmap.clone()),
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
                                    start: None 
                                }
                            ) 
                        },  
                    }
                )  
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct General {
    pub docker: Option<Docker>, 
    pub node: Option<Node>,
    pub file: Amount<File>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub name: Option<String>,
    pub path: Option<String>,
    pub destination: Option<String>,
    pub start: Option<String>
}


