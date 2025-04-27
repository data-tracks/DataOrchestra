use crate::internal::object::object::Object;

use super::process_types::{ProcessType, ProcessTypeConfig};

#[derive(Debug)]
pub struct Process {
    /*
     * Default object information
     */
    pub object: Object,

    /*
     * Process specific information
     */

    /// Process type
    /// Refers to the process type, as available in [`ProcessType`].
    pub process_type: Option<ProcessType>,
    /// Refers to the additional parameters of the structs in [`ProcessType`], like for example [`Kafka`] 
    pub config: Option<ProcessTypeConfig>
}

pub fn default_amount() -> usize {
    1
}
