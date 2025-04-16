use serde::{Deserialize, Serialize};
use crate::{docker::docker_struct::Container, types::{address::Address, amount::Amount}};

use super::store_types::{StoreTypeConfig, StoreType};

/// Represents a store object able to store data. Holds general information for the creation and
/// handling of the storing type.
#[derive(Debug, Deserialize, Serialize)]
pub struct Store {
    /// Relation structure
    pub schema: Option<Amount<String>>,

    /// Docker container
    #[serde(default = "default_docker")]
    pub docker: Container,

    /// Data folder containing all relevant scripts
    pub data: String,

    /// Script that is called upon start
    pub start_script: String,

    /// Store type
    /// Refers to the database type, as available in [`StoreType`].
    #[serde(alias = "type")]
    pub db_type: StoreType,

    /// Refers to the additional parameters of the structs in [`StoreType`], like for example [`PostGres`]
    pub config: Option<StoreTypeConfig>,

    /// Remote connection
    pub remote: Option<Address>,
}

pub fn default_docker() -> Container {
    Container::new()
}
