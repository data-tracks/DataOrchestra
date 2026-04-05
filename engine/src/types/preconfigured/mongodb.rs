use serde::{Deserialize, Serialize};

use crate::traits::Configurable;
use crate::object::Object;

pub fn default_username() -> String {
    String::from("mongo")
}

pub fn default_password() -> String {
    String::from("mongo")
}

/// The `MongoDB` type. Represents the configurability of the MongoDB instance
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename = "mongodb")]
pub struct MongoDB {
    #[serde(default = "default_username")]
    username: String,
    #[serde(default = "default_password")]
    password: String,
}

impl Default for MongoDB {
    fn default() -> MongoDB {
        MongoDB {
            username: default_username(),
            password: default_password(),
        }
    }
}

impl Configurable<Object> for MongoDB {
    fn configure(&mut self, parent: &mut Object) {
        let container = parent
            .docker_container_builder
            .get_or_insert_default();

        container
            .image("mongo:4.4.6")
            .try_name("mongodb")
            .publish_map(27017, 27017)
            .environment("MONGO_INITDB_ROOT_USERNAME", &self.username)
            .environment("MONGO_INITDB_ROOT_PASSWORD", &self.password);
    }
}
