use serde::{Deserialize, Serialize};

use crate::traits::Configurable;
use crate::object::Object;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename = "polypheny")]
pub struct Polypheny {}

impl Default for Polypheny {
    fn default() -> Polypheny {
        Polypheny {}
    }
}

impl Configurable<Object> for Polypheny {
    fn configure(&mut self, _parent: &mut Object) {
        todo!("Not yet implemented")
    }
}
