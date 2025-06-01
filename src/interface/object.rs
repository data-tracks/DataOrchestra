use serde::{Deserialize, Serialize};

use crate::core::object::Object;
use crate::shared::traits::ToInternal;

use super::general::General;

#[derive(Debug, Deserialize, Serialize)]
pub struct ExtObject {
    #[serde(default = "default_amount")]
    pub amount: usize,
    #[serde(flatten)]
    pub general: General
}

pub fn default_amount() -> usize {
    1
}

impl ToInternal<Object> for ExtObject {
    fn to_internal(self) -> Object {
        let mut object = Object::default();

        object.node_data = self.general.node_data.to_internal();
        object.docker_data = self.general.docker_data.to_internal();

        object
    } 
}


