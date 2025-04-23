use serde::{Serialize, Deserialize};
use crate::object::object_struct::Object;
use crate::types::{node::Node, address::Address};


use crate::docker::docker_struct::Container;

#[derive(Debug, Serialize, Deserialize)]
pub struct Generate {
    /*
     * Default object information
     */

    #[serde(flatten)]
    pub object: Object,

    #[serde(default = "default_amount")]
    pub amount: usize,
    pub docker: Option<Container>,
    pub node: Option<Node>,
    pub remote: Option<Address>,
}

pub fn default_amount() -> usize {
    1
}
