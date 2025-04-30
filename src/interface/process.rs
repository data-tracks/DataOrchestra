use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::core::process::process_types::{ProcessType, ProcessTypeConfig};
use crate::shared::{traits::ToInternal, Amount};
use crate::core::process::Process;
use crate::core::adapters::docker::ComposeGroupBuilder;
use crate::core::adapters::docker::container::ContainerBuilder;

use super::config::General;

#[derive(Debug, Deserialize, Serialize)]
pub struct ExtProcess {
    #[serde(rename = "type")]
    pub process_type: Option<ProcessType>,
    #[serde(rename = "config")]
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

        process.process_type = self.process_type;
        process.config = self.config;

        if let Some(docker) = self.general.docker {
            if let Some(compose) = docker.compose {
                let mut builder = ComposeGroupBuilder::new();
                builder.set_compose(compose);
                process.object.docker_group_builder = Some(builder);
            }
            else 
            {
                let mut builder = ContainerBuilder::new();
                if let Some(name) = docker.name {
                    builder.set_name(name);
                }
                if let Some(image) = docker.image {
                    builder.set_image(image);
                }
                if let Some(dockerfile) = docker.dockerfile {
                    builder.set_dockerfile(dockerfile);
                }
                if let Some(build_args) = docker.build_args {
                    for (key, value) in build_args {
                        builder.add_build_arg(key, value);
                    }
                }
                if let Some(network) = docker.network {
                    builder.set_network(network);
                }
                if let Some(env) = docker.enviroment {
                    for (key, value) in env {
                        builder.add_env_var(key, value);
                    }
                }
                match docker.mount {
                    Amount::Single(mount) => {
                        builder.add_mount(mount);
                    }
                    ,
                    Amount::Multiple(mounts) => {
                        for mount in mounts {
                            builder.add_mount(mount);
                        }
                    },
                    Amount::None => ()
                }

                builder.set_publish_all(docker.publish_all);

                process.object.docker_container_builder = Some(builder);
            } 
        }

        process
    }
}
