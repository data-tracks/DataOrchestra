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

        // If a process type is given prioritise this over additional docker config
        if let Some(docker) = self.general.docker {
            if docker.compose.is_some() {
                process.object.docker_group_builder = Some(docker.to_internal());
            }
            else 
            {
                process.object.docker_container_builder = Some(docker.to_internal());
            } 
        } 
        
        if self.general.node.is_some() {
            process.object.node = Some(self.general.node.unwrap().to_internal());
        } 
        
        process.object.data = self.general.file.to_internal();

        process
    }
}
