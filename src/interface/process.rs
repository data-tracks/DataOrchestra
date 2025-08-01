use serde::{Deserialize, Serialize};
use crate::core::process::process_types::{ProcessType, ProcessTypeConfig};
use crate::core::process::Process;
use crate::shared::traits::ToInternal;
use super::general::General;

/// External representation of the internal [`Process`] object
#[derive(Debug, Deserialize, Serialize)]
pub struct ExtProcess {
    #[serde(rename = "type")]
    pub process_type: Option<ProcessType>,
    #[serde(flatten)]
    pub config: Option<ProcessTypeConfig>,
    #[serde(flatten)]
    pub general: General
}

impl Default for ExtProcess {
    fn default() -> Self {
        ExtProcess 
        { 
            process_type: None, 
            config: None, 
            general: General::default()
        }
    }
}

impl ToInternal<Process> for ExtProcess {
    fn to_internal(self) -> Process {
        let mut process = Process::default();

        process.object.name = self.general.name.unwrap_or("process".to_string());

        process.object.graph = self.general.graph;

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
            if docker.compose.is_some() {
                process.object.docker_group_builder = Some(docker.to_internal());
            }
            else 
            {
                process.object.docker_container_builder = Some(docker.to_internal());
            } 
        } 

        process.object.resources = self.general.resources.to_internal();
        let vec = self.general.executables.clone().to_internal();
        for (script, data) in vec {
            if let Some(data) = data {
                process.object.resources.push(data);
            }
            process.object.executables.push(script);
        }

        process
    }
}
