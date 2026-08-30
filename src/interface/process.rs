use super::general::General;
use crate::core::process::Process;
use crate::core::process::process_types::{ProcessType, ProcessTypeConfig};
use crate::interface::docker::ExtDocker;
use crate::shared::traits::ToInternal;
use serde::{Deserialize, Serialize};

/// External representation of the internal [`Process`] object
#[derive(Debug, Deserialize, Serialize)]
pub struct ExtProcess {
    #[serde(rename = "type")]
    pub process_type: Option<ProcessType>,
    #[serde(flatten)]
    pub config: Option<ProcessTypeConfig>,
    #[serde(flatten)]
    pub general: General,
}

impl Default for ExtProcess {
    fn default() -> Self {
        ExtProcess {
            process_type: None,
            config: None,
            general: General::default(),
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

        if let Some(docker) = self.general.docker.clone() {
            match docker {
                ExtDocker::Compose(compose) => {
                    process.object.docker_group_builder = Some(compose.to_internal());
                }
                ExtDocker::Container(container) => {
                    process.object.docker_container_builder = Some(container.to_internal());
                }
                ExtDocker::Dind(dind) => {
                    process.object.docker_container_builder = Some(dind.to_internal());
                }
            };
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
