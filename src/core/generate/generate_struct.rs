use serde::{Serialize, Deserialize};
use crate::object::object_struct::Object;
use crate::types::{node::Node, address::Address};


use crate::docker::docker_struct::Data;

#[derive(Debug)]
pub struct Generate {
    /*
     * Default object information
     */
    pub object: Object,
    pub amount: usize,
    pub docker: Option<Data>,
    pub node: Option<Node>,
    pub remote: Option<Address>,
}

pub fn default_amount() -> usize {
    1
}
