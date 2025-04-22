use serde::{Deserialize, Serialize};
use crate::{object::object_struct::Object, types::amount::Amount};

use super::store_types::{StoreTypeConfig, StoreType};

/// Represents a store object able to store data. Holds general information for the creation and
/// handling of the storing type.
#[derive(Debug, Deserialize, Serialize)]
pub struct Store {
    /*
     * Default object information
     */

    #[serde(flatten)]
    pub object: Object,

    /*
     * Store specific information
     */

    /// Relation structure
    #[serde(default)]
    pub schema: Amount<String>,
    
    /// Store type
    /// Refers to the database type, as available in [`StoreType`].
    #[serde(alias = "type")]
    pub db_type: Option<StoreType>,

    /// Refers to the additional parameters of the structs in [`StoreType`], like for example [`PostGres`]
    pub config: Option<StoreTypeConfig>,
}
