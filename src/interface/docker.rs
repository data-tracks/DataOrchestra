use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::shared::Amount;


#[derive(Debug, Deserialize, Serialize)]
pub struct Docker {
    // Name of container
    pub name: Option<String>,
    // Network of container
    pub network: Option<String>,
    // Additional options of container
    pub options: Option<HashMap<String, String>>,
    // Mounts of container
    pub mount: Amount<String>,
    // Publish all ports
    pub publish_all: bool,
    // How container(s) are created
    pub image: Option<String>,
    pub dockerfile: Option<String>,
    pub build_args: Option<HashMap<String, String>>,
    pub compose: Option<String>,
}
