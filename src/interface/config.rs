use log::debug;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::Error;
use serde_json::Value;
use crate::core::adapters::portainer::Portainer;
use crate::core::config::Config;
use crate::core::object::Object;
use crate::core::traits::Creator;
use crate::shared::{Amount, ToInternal, ToInternalVec};
use super::api::API;
use super::store::ExtStore;
use super::process::ExtProcess;
use super::object::ExtObject;
use super::generate::ExtGenerate;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtConfig {
    pub api: API,
    #[serde(default)]
    pub portainer: Portainer,
    #[serde(default)]
    pub generate: Amount<ExtGenerate>,
    #[serde(default)]
    pub process: Amount<ExtProcess>,
    #[serde(default)]
    pub store: Amount<ExtStore>,
    #[serde(default)]
    pub object: Amount<ExtObject>,
}

impl ToInternal<(Config, Portainer)> for ExtConfig {
    fn to_internal(self) -> (Config, Portainer) {
        let (process, consumer) = self.api.to_internal();

        let mut config = Config 
        {
            generate: self.generate.to_internal(),
            process: self.process.to_internal(),
            store: self.store.to_internal(),
            object: self.object.to_internal(),
        };

        config.process.push(process);
        config.object.push(consumer);

        (config, self.portainer)
    }
}

impl ExtConfig {
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