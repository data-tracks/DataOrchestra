use serde::{Deserialize, Serialize};
use crate::core::process::process_types::{ProcessType, ProcessTypeConfig};
use crate::core::process::Process;
use crate::shared::traits::ToInternal;
use super::general::General;


#[derive(Debug, Deserialize, Serialize)]
pub struct ExtProcess {
    #[serde(rename = "type")]
    pub process_type: Option<ProcessType>,
    #[serde(flatten)]
    pub config: Option<ProcessTypeConfig>,
    #[serde(default = "default_amount")]
    pub amount: usize,
    #[serde(flatten)]
    pub general: General
}

pub fn default_amount() -> usize {
    1
}

impl ToInternal<Process> for ExtProcess {
    fn to_internal(self) -> Process {
        let mut process = Process::default();
        process.process_type = self.process_type;
        process.config = self.config;

        if let Some(node) = self.general.node {
            process.object.node = Some(node.to_internal());
        }

        if let Some(ansible) = self.general.ansible {
            process.object.ansible = ansible;
        }

        // If a process type is given prioritise this over additional docker config
        if let Some(docker) = self.general.docker {
            if docker.compose.is_some() || docker.names.is_some() {
                process.object.docker_group_builder = Some(docker.to_internal());
            }
            else 
            {
                process.object.docker_container_builder = Some(docker.to_internal());
            } 
        } 

        process.object.node_data = self.general.node_data.to_internal();
        process.object.docker_data = self.general.docker_data.to_internal();

        process
    }
}
