use serde::{Deserialize, Serialize};

use crate::core::{object::Object, traits::Configurator};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename = "polypheny")]
pub struct Polypheny {}

impl Default for Polypheny {
    fn default() -> Polypheny {
        Polypheny {}
    }
}

impl Configurator<Object> for Polypheny {
    fn configure(&mut self, parent: &mut Object) {
        todo!("Not yet implemented")
    }
}
