use log::debug;
use serde::{Deserialize, Serialize};
use crate::{core::generate::{generate_types::{GeneratorTypeConfig}, Generate}, shared::traits::{ToInternal, ToInternalVec}};
use crate::core::object::Object;
use crate::interface::object::ExtObject;

/// External representation of the internal [`Generate`] object
#[derive(Debug, Deserialize, Serialize)]
pub struct ExtGenerate {
    #[serde(flatten)]
    pub config: Option<GeneratorTypeConfig>,
    #[serde(default = "ExtGenerate::default_amount")]
    pub amount: usize,
    #[serde(flatten)]
    pub object: ExtObject
}


impl Default for ExtGenerate {
    fn default() -> Self {
        ExtGenerate 
        {
            //generate_type: None,
            config: None,
            amount: ExtGenerate::default_amount(),
            object: ExtObject::default()
        }
    }
}

impl ExtGenerate {
    pub fn default_amount() -> usize {
        1
    }
}

impl ToInternalVec<Generate> for ExtGenerate {
    fn to_internal(self) -> Vec<Generate> {
        let mut vec_generate = Vec::<Generate>::new();
        for i in 0..self.amount {
            let mut generate = Generate::default();

            generate.object = self.object.clone().to_internal();

            if generate.object.name.eq(&Object::default_name()) {
                generate.object.name = "generate".to_string();
            }

            if self.amount > 0 {
                generate.object.name = format!("{}-{i}", generate.object.name);
            }

            generate.config = self.config.clone();

            if let Some(docker) = generate.object.docker_container_builder.as_mut() {
                if let Some(name) = docker.get_name_mut() {
                    *name = format!("{name}-{i}");
                }
            }

            debug!("Finished parsing generate to internal");
            vec_generate.push(generate);
        }

        vec_generate
    }
}
