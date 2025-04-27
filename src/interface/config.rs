use serde::{Serialize, Deserialize};

use crate::shared::{Amount, Node};

use super::{docker::Docker, generate::ExtGenerate, object::ExternalObject, process::ExtProcess, store::ExtStore};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub generate: Amount<ExtGenerate>,
    pub process: Amount<ExtProcess>,
    pub store: Amount<ExtStore>,
    pub object: Amount<ExternalObject>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct General {
    docker: Option<Docker>, 
    node: Option<Node>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    // Specific for docker implementation
    name: Option<String>,
    path: Option<String>,
    destination: Option<String>,
    start: Option<String>
}


