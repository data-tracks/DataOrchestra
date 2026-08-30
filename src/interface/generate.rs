use crate::{
    core::generate::{Generate, generate_types::GeneratorTypeConfig},
    interface::docker::ExtDocker,
    shared::traits::{ToInternal, ToInternalVec},
};
use log::debug;
use serde::{Deserialize, Serialize};

use super::general::General;

/// External representation of the internal [`Generate`] object
#[derive(Debug, Deserialize, Serialize)]
pub struct ExtGenerate {
    //#[serde(rename = "type")]
    //pub generate_type: Option<GeneratorType>,
    #[serde(flatten)]
    pub config: Option<GeneratorTypeConfig>,
    #[serde(default = "default_amount")]
    pub amount: usize,
    #[serde(flatten)]
    pub general: General,
}

pub fn default_amount() -> usize {
    1
}

impl Default for ExtGenerate {
    fn default() -> Self {
        ExtGenerate {
            //generate_type: None,
            config: None,
            amount: default_amount(),
            general: General::default(),
        }
    }
}

impl ToInternalVec<Generate> for ExtGenerate {
    fn to_internal(self) -> Vec<Generate> {
        let mut vec_generate = Vec::<Generate>::new();
        for i in 0..self.amount {
            let mut generate = Generate::default();

            generate.object.name = self.general.name.clone().unwrap_or("generate".to_string());

            if self.amount > 0 {
                generate.object.name = format!("{}-{i}", generate.object.name);
            }
            generate.object.graph = self.general.graph.clone();

            //generate.generate_type = self.generate_type.clone();
            generate.config = self.config.clone();

            if let Some(node) = self.general.node.clone() {
                generate.object.node = Some(node.to_internal());
            }

            if let Some(ansible) = self.general.ansible.clone() {
                generate.object.ansible = ansible;
            }

            if let Some(docker) = self.general.docker.clone() {
                match docker {
                    ExtDocker::Compose(compose) => {
                        generate.object.docker_group_builder = Some(compose.to_internal());
                    }
                    ExtDocker::Container(mut container) => {
                        if let Some(name) = container.name {
                            container.name = Some(format!("{name}-{i}"));
                        }
                        generate.object.docker_container_builder = Some(container.to_internal());
                    }
                    ExtDocker::Dind(dind) => {
                        generate.object.docker_container_builder = Some(dind.to_internal());
                    }
                };
            }

            generate.object.resources = self.general.resources.clone().to_internal();
            let vec = self.general.executables.clone().to_internal();
            for (script, data) in vec {
                if let Some(data) = data {
                    generate.object.resources.push(data);
                }
                generate.object.executables.push(script);
            }

            debug!("Finished parsing generate to internal");

            vec_generate.push(generate);
        }

        vec_generate
    }
}
