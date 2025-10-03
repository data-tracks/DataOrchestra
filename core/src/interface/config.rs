use std::fs;
use std::path::Path;

use super::generate::ExtGenerate;
use super::object::ExtObject;
use super::process::ExtProcess;
use super::store::ExtStore;
use crate::config::Config;
use crate::object::Object;
use crate::shared::{Amount, ToInternal, ToInternalVec};
use crate::traits::Creator;
use crate::variables::variables::Variables;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Types {
    Object(ExtObject),
    Generate(ExtGenerate),
    Process(ExtProcess),
    Store(ExtStore),
}

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
    pub generate: Amount<ExtGenerate>,
    #[serde(default)]
    pub process: Amount<ExtProcess>,
    #[serde(default)]
    pub store: Amount<ExtStore>,
    #[serde(default)]
    pub object: Amount<ExtObject>,
}

impl ToInternal<Config> for ExtConfig {
    fn to_internal(self) -> Config {
        Config {
            generate: self.generate.to_internal(),
            process: self.process.to_internal(),
            store: self.store.to_internal(),
            object: self.object.to_internal(),
        }
    }
}

impl ExtConfig {
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
        for object in self.object.as_mut_ref_vec() {
            for config in object.attach_config.take().to_vec() {
                attach_objects.push(config.create(&object));
            }
        }

        for store in self.store.as_mut_ref_vec() {
            for config in store.object.attach_config.take().to_vec() {
                attach_objects.push(config.create(&store.object));
            }
        }

        for process in self.process.as_mut_ref_vec() {
            for config in process.object.attach_config.take().to_vec() {
                attach_objects.push(config.create(&process.object));
            }
        }

        for generate in self.generate.as_mut_ref_vec() {
            for config in generate.object.attach_config.take().to_vec() {
                attach_objects.push(config.create(&generate.object));
            }
        }

        attach_objects
    }
}
