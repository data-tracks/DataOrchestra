use crate::types::object::{ExtObject};
use log::debug;
use serde::{Deserialize, Serialize};
use data_orchestra_engine::object::Object;
use data_orchestra_engine::process::Process;
use data_orchestra_engine::process::process_types::{ProcessType, ProcessTypeConfig};
use crate::traits::ToInternal;

/// External representation of the internal [`Process`] object
#[derive(Debug, Deserialize, Serialize)]
pub struct ExtProcess {
    #[serde(rename = "type")]
    pub process_type: Option<ProcessType>,
    #[serde(flatten)]
    pub config: Option<ProcessTypeConfig>,
    #[serde(flatten)]
    pub object: ExtObject,
}

impl Default for ExtProcess {
    fn default() -> Self {
        ExtProcess {
            process_type: None,
            config: None,
            object: ExtObject::default(),
        }
    }
}

impl ToInternal<Process> for ExtProcess {
    fn to_internal(self) -> Process {
        let mut process = Process::default();

        process.object = self.object.to_internal();

        if process.object.name.eq(&Object::default_name()) {
            process.object.name = "process".to_string();
        }

        process.process_type = self.process_type;
        process.config = self.config;

        debug!("Finished parsing process to internal");
        process
    }
}
