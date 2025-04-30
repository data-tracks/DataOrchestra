use serde::{Deserialize, Serialize};

use crate::{core::object::Object, shared::traits::ToInternal};

use super::config::General;

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

        object.files = self.general.file.to_vec();

        object
    } 
}


