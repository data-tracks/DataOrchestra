use serde::{Deserialize, Serialize};

use crate::object::object_struct::Object;

use super::process_types::{ProcessType, ProcessTypeConfig};

#[derive(Debug, Deserialize, Serialize)]
pub struct Process {
    /*
     * Default object information
     */

    #[serde(flatten)]
    pub object: Object,

    /*
     * Process specific information
     */

    #[serde(default = "default_amount")]
    pub amount: usize,

    /// Process type
    /// Refers to the process type, as available in [`ProcessType`].
    #[serde(rename = "type")]
    pub process_type: Option<ProcessType>,
    /// Refers to the additional parameters of the structs in [`ProcessType`], like for example [`Kafka`] 
    pub config: Option<ProcessTypeConfig>
}

pub fn default_amount() -> usize {
    1
}
