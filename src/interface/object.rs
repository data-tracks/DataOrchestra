use serde::{Deserialize, Serialize};

use crate::core::object::Object;
use crate::shared::traits::ToInternal;

use super::general::General;

/// External representation of the internal [`Object`] object
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExtObject {
    #[serde(flatten)]
    pub general: General
}

impl Default for ExtObject {
    fn default() -> Self {
        ExtObject 
        { 
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
            if docker.compose.is_some() {
                object.docker_group_builder = Some(docker.to_internal());
            }
            else 
            {
                object.docker_container_builder = Some(docker.to_internal());
            } 
        }

        object.resources = self.general.resources.to_internal();
        let vec = self.general.executables.clone().to_internal();
        for (script, data) in vec {
            if let Some(data) = data {
                object.resources.push(data);
            }
            object.executables.push(script);
        }

        object
    } 
}


