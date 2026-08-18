use super::api::API;
use super::object::ExtObject;
use crate::core::adapters::portainer::portainer::Portainer;
use crate::core::config::Config;
use crate::core::object::Object;
use crate::core::traits::Creator;
use crate::shared::{Amount, ToInternal};
use log::debug;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ExtConfig {
    pub api: API,
    #[serde(default)]
    pub portainer: Portainer,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_object")]
    pub object: Amount<ExtObject>,
}

impl ToInternal<(Config, Portainer)> for ExtConfig {
    fn to_internal(self) -> (Config, Portainer) {
        let (kafka_api, kafka_consumer) = self.api.to_internal();

        let mut config = Config {
            objects: self.object.to_internal(),
        };

        config.objects.push(kafka_consumer);
        //config.object.push(api);
        let portainer = self.portainer;

        (config, portainer)
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

        attach_objects
    }
}

pub fn deserialize_object<'de, D>(deserializer: D) -> Result<Amount<ExtObject>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    let first = value.to_string();

    if first.chars().next().is_some_and(|x| x == '[') {
        let result = Vec::deserialize(value.clone());
        if let Ok(compact) = result {
            return Ok(Amount::Multiple(compact));
        } else if let Err(error) = result {
            debug!("{}", value.clone());
            panic!("Error while deserializing Vec<object> [{}]", error);
        }
    } else {
        let result = ExtObject::deserialize(value.clone());
        if let Ok(full) = result {
            return Ok(Amount::Single(full));
        } else if let Err(error) = result {
            debug!("{}", value.clone());
            panic!("Error while deserializing object: [{}]", error);
        }
    }

    Err(serde::de::Error::custom(
        "Could not deserialize into either a single object or a list of them",
    ))
}
