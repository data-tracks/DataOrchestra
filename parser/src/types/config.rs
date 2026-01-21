use crate::traits::ToInternalVec;
use std::fs;
use std::path::Path;
use crate::types::generate::ExtGenerate;
use crate::types::object::ExtObject;
use crate::types::process::ExtProcess;
use crate::types::store::ExtStore;
use crate::variables::variables::Variables;
use serde::{Deserialize, Serialize};
use data_orchestra_engine::config::Config;
use data_orchestra_engine::object::Object;
use crate::amount::Amount;
use crate::traits::{Creatable, ToInternal};
use crate::types::upload::UploadTypes;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Types {
    Object(ExtObject),
    Generate(ExtGenerate),
    Process(ExtProcess),
    Store(ExtStore),
}

impl Types {
    fn split(types: Vec<Self>) -> (Vec<ExtObject>, Vec<ExtGenerate>, Vec<ExtProcess>, Vec<ExtStore>) {
        let ext_objects = Vec::new();
        let ext_generate = Vec::new();
        let ext_process = Vec::new();
        let ext_store = Vec::new();

        (ext_objects, ext_generate, ext_process, ext_store)
    }
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
    pub components: Amount<Types>,
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
    fn to_internal(mut self) -> Config {
        let attachables = self.extract_attachables();

        //let components = self.components.

        let generate = self.generate.to_internal();
        let process = self.process.to_internal();
        let store = self.store.to_internal();
        let mut object = self.object.to_internal();
        object.extend(attachables);

        Config {
            store,
            process,
            generate,
            object,
            agents: Vec::new(),
        }
    }
}

impl ExtConfig {
    pub fn set_uploader(&mut self, uploader: UploadTypes) {
        for object in self.object.as_mut_vec().iter_mut() {
            if let Some(node) = object.node.as_mut() {
                node.upload_schema = uploader.clone();
            }
        }

        for store in self.store.as_mut_vec().iter_mut() {
            if let Some(node) = store.object.node.as_mut() {
                node.upload_schema = uploader.clone();
            }
        }

        for process in self.process.as_mut_vec().iter_mut() {
            if let Some(node) = process.object.node.as_mut() {
                node.upload_schema = uploader.clone();
            }
        }

        for generate in self.generate.as_mut_vec().iter_mut() {
            if let Some(node) = generate.object.node.as_mut() {
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
        for object in self.object.as_mut_vec() {
            for config in object.attach_config.take().to_vec() {
                attach_objects.push(config.create(&object));
            }
        }

        for store in self.store.as_mut_vec() {
            for config in store.object.attach_config.take().to_vec() {
                attach_objects.push(config.create(&store.object));
            }
        }

        for process in self.process.as_mut_vec() {
            for config in process.object.attach_config.take().to_vec() {
                attach_objects.push(config.create(&process.object));
            }
        }

        for generate in self.generate.as_mut_vec() {
            for config in generate.object.attach_config.take().to_vec() {
                attach_objects.push(config.create(&generate.object));
            }
        }

        attach_objects
    }
}
