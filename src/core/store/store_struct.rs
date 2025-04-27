use std::thread::JoinHandle;

use serde::{Deserialize, Serialize};

use crate::internal::object::object::Object;
use crate::{common::common_trait::Start, types::amount::Amount};
use super::super::object;

use super::store_types::{StoreTypeConfig, StoreType};

/// Represents a store object able to store data. Holds general information for the creation and
/// handling of the storing type.
#[derive(Debug)]
pub struct Store {
    /*
     * Default object information
     */
    pub object: Object,

    /*
     * Store specific information
     */

    /// Relation structure
    pub schema: Amount<String>,
    
    /// Store type
    /// Refers to the database type, as available in [`StoreType`].
    pub db_type: Option<StoreType>,

    /// Refers to the additional parameters of the structs in [`StoreType`], like for example [`PostGres`]
    pub config: Option<StoreTypeConfig>,
}

impl Default for Store {
    fn default() -> Self {
        Store {
            object: Object::default(),
            schema: Amount::None,
            db_type: None,
            config: None
        }
    }
}

impl Amount<Store> {
    pub fn start(self) -> Vec<JoinHandle<()>> {
        let mut threads = Vec::<JoinHandle<()>>::new();
        match self {
            Amount::Single(store) => threads.push(store.start()),
            Amount::Multiple(stores) => {
                for store in stores {
                    threads.push(store.start());
                }
            }
            Amount::None => ()
        } 

        threads
    }
}
