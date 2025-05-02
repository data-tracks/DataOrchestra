use serde::{Deserialize, Serialize};
use crate::core::adapters::docker::ComposeGroupBuilder;
use crate::core::process::process_types::{ProcessType, ProcessTypeConfig};
use crate::shared::{traits::ToInternal, Amount};
use crate::core::process::Process;
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

impl ToInternal<Amount<Process>> for Amount<ExtProcess> {
    fn to_internal(self) -> Amount<Process> {
        match self {
            Amount::None => Amount::None,
            Amount::Single(process) => Amount::Single(process.to_internal()),
            Amount::Multiple(processes) => {
                let mut vec_process = Vec::<Process>::new();
                for process in processes {
                    vec_process.push(process.to_internal());
                };

                Amount::Multiple(vec_process)
            }
        }
    }
}

impl ToInternal<Process> for ExtProcess {
    fn to_internal(self) -> Process {
        let mut process = Process::default();
        dbg!(&self.config);
        process.process_type = self.process_type;
        process.config = self.config;

        // If a process type is given prioritise this over additional docker config
        if process.process_type.is_some() {
            let mut builder = ComposeGroupBuilder::new();
            if process.config.is_none() {
                dbg!(&process.process_type);
                process.config = Some(process.process_type.as_ref().unwrap().new());
            }

            process.config.as_ref().unwrap().setup_container(&mut builder);
        }
        else {
            if let Some(docker) = self.general.docker {
                if docker.compose.is_some() {
                    process.object.docker_group_builder = Some(docker.to_internal());
                }
                else 
                {
                    process.object.docker_container_builder = Some(docker.to_internal());
                } 
            } 
        }

        process.object.node = self.general.node;
        process.object.data = self.general.file.to_internal();

        process
    }
}
