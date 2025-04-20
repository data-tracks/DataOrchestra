use serde::{Deserialize, Serialize};

use crate::{docker::docker_struct::Container, types::{address::Address, node::Node}};

#[derive(Debug, Deserialize, Serialize)]
pub struct Object {
    #[serde(rename = "docker")]
    pub docker: Option<Container>,
    #[serde(rename = "start")]
    pub start: Option<String>,
    #[serde(rename = "data")]
    pub data: Option<String>,
    #[serde(rename = "node")]    
    pub node: Option<Node>,

    #[serde(rename = "remote")]
    #[serde(skip)]
    pub remote: Option<Address>,
}
