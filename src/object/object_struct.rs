use serde::{Deserialize, Serialize};

use crate::{docker::docker_struct::Container, types::{address::Address, amount::Amount, node::Node}};

use super::attach::attach_types::AttachType;

/// The `Object` type. Acts as a generic component. Implements basic fields that every object
/// should possess.
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

    #[serde(rename = "attach_type")]
    pub attach_type: Option<AttachType>,
    #[serde(rename = "attach")]
    pub attach: Amount<Box<Object>>
}
