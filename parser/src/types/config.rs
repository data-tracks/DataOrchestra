use std::fs;
use std::path::Path;
use crate::types::object::ExtObject;
use crate::variables::variables::Variables;
use serde::{Deserialize, Serialize};
use data_orchestra_engine::config::Config;
use data_orchestra_engine::object::Object;
use crate::amount::Amount;
use crate::traits::{Creatable, ToInternal};
use crate::types::upload::UploadTypes;


/// External configuration type. Represents the external interface config of the internal [Config] type.
///
/// # Fields
///
/// The object types are wrapped into [shared::Amount] to allow for none, single or multiple items
/// to be specified in the JSON.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtConfig {
    #[serde(default)]
    pub objects: Amount<ExtObject>,
}

impl ToInternal<Config> for ExtConfig {
    fn to_internal(mut self) -> Config {
        let attachables = self.extract_attachables();
        let mut object = self.objects.to_internal();
        object.extend(attachables);

        Config {
            objects: object,
            agents: Vec::new(),
        }
    }
}

impl ExtConfig {
    pub fn set_uploader(&mut self, uploader: UploadTypes) {
        for object in self.objects.as_mut_vec().iter_mut() {
            if let Some(node) = object.node.as_mut() {
                node.upload_schema = uploader.clone();
            }
        }
    }

    /// Parse file location to external configuration type
    pub fn parse(path: &Path) -> ExtConfig {
        let config = fs::read_to_string(path).expect("Unable to read config file");
        let variables: Variables =
            serde_json::from_str(config.as_str()).expect("Unable to parse config to struct");
        let ext_config_string = variables.parse(config);

        serde_json::from_str(ext_config_string.as_str()).expect("Unable to parse config to struct")
    }

    /// Extract the attachable components from the different components and parse them into their
    /// own objects
    pub fn extract_attachables(&mut self) -> Vec<Object> {
        let mut attach_objects = Vec::new();
        for object in self.objects.as_mut_vec() {
            for config in object.attach_config.take().to_vec() {
                attach_objects.push(config.create(&object));
            }
        }

        attach_objects
    }
}
