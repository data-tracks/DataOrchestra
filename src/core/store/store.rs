use crate::core::object::Object;

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
    pub schema: Vec<String>,
    
    /// Store type
    /// Refers to the database type, as available in [`StoreType`].
    pub db_type: Option<StoreType>,

    /// Refers to the additional parameters of the structs in [`StoreType`], like for example [`PostGres`]
    pub config: Option<StoreTypeConfig>,
}

impl Default for Store {
    fn default() -> Self {
        Store
        {
            object: Object::default(),
            schema: Vec::new(),
            db_type: None,
            config: None
        }
    }
}
