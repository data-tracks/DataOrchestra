use serde::{Serialize, Deserialize};

use crate::object::object_struct::Object;
use crate::object::store::store_struct::Store;
use crate::object::process::process_struct::Process;
use crate::object::generate::generate_struct::Generate;

use super::amount::Amount;
use super::node::Node;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Config {
    /// All available nodes
    #[serde(default)]
    pub nodes: Amount<Node>,
    /// All processing object
    #[serde(default)]
    pub process: Amount<Process>,
    /// All generating object
    #[serde(default)]
    pub generate: Amount<Generate>,
    /// All storing objects
    #[serde(default)]
    pub store: Amount<Store>,
    /// All other, generic objects
    #[serde(default)]
    pub object: Amount<Object>
}
