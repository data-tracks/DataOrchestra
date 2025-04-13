use serde::{Serialize, Deserialize};
use crate::types::{node::Node, address::Address};


use crate::docker::docker_struct::Docker;

#[derive(Debug, Serialize, Deserialize)]
pub struct Generate {
    #[serde(default = "default_amount")]
    pub amount: usize,
    pub docker: Option<Docker>,
    pub node: Option<Node>,
    pub remote: Option<Address>,
    pub script: String
}

pub fn default_amount() -> usize {
    1
}
