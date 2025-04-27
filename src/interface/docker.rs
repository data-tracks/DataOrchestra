use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::shared::Amount;


#[derive(Debug, Deserialize, Serialize)]
pub struct Docker {
    // Name of container
    name: Option<String>,
    // Network of container
    network: Option<String>,
    // Additional options of container
    options: Option<HashMap<String, String>>,
    // Mounts of container
    mount: Amount<String>,
    // Publish all ports
    publish_all: bool,
    // How container(s) are created
    #[serde(flatten)]
    create_type: CreateType,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[serde(rename_all = "lowercase")]
pub enum CreateType {
    Image { image: String },
    Dockerfile { 
        dockerfile: String, 
        image_name: String, 
        build_args: Option<HashMap<String, String>> 
    },
    Compose { compose: String }
}
