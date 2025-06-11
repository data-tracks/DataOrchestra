use log::debug;
use serde::{Deserialize, Serialize};
use crate::{core::generate::{generate_types::{GeneratorType, GeneratorTypeConfig}, Generate}, shared::traits::{ToInternal, ToInternalVec}};

use super::general::General;

#[derive(Debug, Deserialize, Serialize)]
pub struct ExtGenerate {
    #[serde(rename = "type")]
    pub generate_type: Option<GeneratorType>,
    #[serde(flatten)]
    pub config: Option<GeneratorTypeConfig>,
    #[serde(default = "default_amount")]
    pub amount: usize,
    #[serde(flatten)]
    pub general: General
}

pub fn default_amount() -> usize {
    1
}

impl Default for ExtGenerate {
    fn default() -> Self {
        ExtGenerate 
        {
            generate_type: None,
            config: None,
            amount: default_amount(),
            general: General::default()
        }
    }
}

impl ToInternalVec<Generate> for ExtGenerate {
    fn to_internal(self) -> Vec<Generate> {
        let mut vec_generate = Vec::<Generate>::new();
        for i in 0..self.amount {
            let mut generate = Generate::default();

            generate.object.name = self.general.name.clone().unwrap_or("store".to_string());

            generate.object.graph.to = self.general.graph.to.clone().to_vec();

            if let Some(ref config) = self.config {
                generate.object.docker_data.push(config.create());
            }
            else if let Some(ref generator_type) = self.generate_type {
                let config = generator_type.new();
                generate.object.docker_data.push(config.create());
            }

            if let Some(node) = self.general.node.clone() {
                generate.object.node = Some(node.to_internal());
            } 

            if let Some(ansible) = self.general.ansible.clone() {
                generate.object.ansible = ansible;
            }

            if let Some(mut docker) = self.general.docker.clone() {
                if docker.compose.is_some() || docker.names.is_some() {
                    generate.object.docker_group_builder = Some(docker.to_internal());
                }
                else 
                {
                    if let Some(name) = docker.name {
                        docker.name = Some(format!("{}-{}", name, i)); 
                    }
                    generate.object.docker_container_builder = Some(docker.to_internal());
                } 
            } 

            generate.object.node_data = self.general.node_data.clone().to_internal();
            generate.object.docker_data.extend(self.general.docker_data.clone().to_internal());

            debug!("Finished parsing generate to internal");

            vec_generate.push(generate);
        }

        vec_generate
    }
}
