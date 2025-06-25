use serde::{Deserialize, Serialize};

use crate::core::{store::Store, traits::Configurator};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "polypheny")]
pub struct Polypheny {}

impl Default for Polypheny {
    fn default() -> Polypheny {
        Polypheny {  }
    }
}

impl Configurator<Store> for Polypheny {
    fn configure(&mut self, parent: &mut Store) {
        todo!("Not yet implemented")
    }
}
