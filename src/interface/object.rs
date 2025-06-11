use serde::{Deserialize, Serialize};

use crate::core::object::Object;
use crate::shared::traits::ToInternal;

use super::general::General;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExtObject {
    #[serde(default = "default_amount")]
    pub amount: usize,
    #[serde(flatten)]
    pub general: General
}

pub fn default_amount() -> usize {
    1
}

impl Default for ExtObject {
    fn default() -> Self {
        ExtObject 
        { 
            amount: default_amount(), 
            general: General::default() 
        }
    }
}

impl ToInternal<Object> for ExtObject {
    fn to_internal(self) -> Object {
        let mut object = Object::default();

        object.name = self.general.name.unwrap_or("object".to_string());

        object.graph = self.general.graph;

        if let Some(node) = self.general.node {
            object.node = Some(node.to_internal());
        }

        if let Some(ansible) = self.general.ansible {
            object.ansible = ansible;
        }

        // Set Container(s) builder
        if let Some(docker) = self.general.docker {
            if docker.compose.is_some() || docker.names.is_some() {
                object.docker_group_builder = Some(docker.to_internal());
            }
            else 
            {
                object.docker_container_builder = Some(docker.to_internal());
            } 
        }

        object.node_data = self.general.node_data.to_internal();
        object.docker_data = self.general.docker_data.to_internal();

        object
    } 
}


