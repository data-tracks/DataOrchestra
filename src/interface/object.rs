use serde::{Deserialize, Serialize};

use crate::{core::{data::{Data}, object::Object}, shared::traits::ToInternal};

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

        let mut data_vec = Vec::<Data>::new();
        for file in self.general.file.to_vec() {
            data_vec.push(file.to_internal());
        }
        object.data = data_vec;

        object
    } 
}


