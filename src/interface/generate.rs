use log::debug;
use serde::{Deserialize, Serialize};
use crate::{core::generate::Generate, shared::traits::{ToInternalVec, ToInternal}};

use super::general::General;

#[derive(Debug, Deserialize, Serialize)]
pub struct ExtGenerate {
    #[serde(default = "default_amount")]
    pub amount: usize,
    #[serde(flatten)]
    pub general: General
}

pub fn default_amount() -> usize {
    1
}

impl ToInternalVec<Generate> for ExtGenerate {
    fn to_internal(self) -> Vec<Generate> {
        let mut vec_generate = Vec::<Generate>::new();
        
        for _i in 0..self.amount {
            let mut generate = Generate::default();
            
            if self.general.node.is_some() {
                generate.object.node = Some(self.general.node.clone().unwrap().to_internal());
            } 

            if let Some(docker) = self.general.docker.clone() {
                if docker.compose.is_some() {
                    generate.object.docker_group_builder = Some(docker.to_internal());
                }
                else 
                {
                    generate.object.docker_container_builder = Some(docker.to_internal());
                } 
            } 

            generate.object.data = self.general.file.clone().to_internal();

            debug!("Finished parsing generate to internal");

            vec_generate.push(generate);
        }

        vec_generate
    }
}
