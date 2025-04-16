use serde::{Deserialize, Serialize};

use crate::{docker::docker_struct::Container, types::amount::Amount};

pub fn default_username() -> String {
    String::from("mongo")
}

pub fn default_password() -> String {
    String::from("mongo")
}

pub fn default_url() -> String {
    String::from("mongodb://mongo:example@mongo:27017")
}

pub fn default_auth() -> bool {
    false
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "mongodb")]
pub struct MongoDB {
    #[serde(default = "default_username")]
    me_config_mongodb_adminusername: String,
    #[serde(default = "default_password")]
    me_config_mongodb_adminpassword: String,
    #[serde(default = "default_url")]
    me_config_mongodb_url: String,
    #[serde(default = "default_auth")]
    me_config_basicauth: bool
}

impl MongoDB {
    pub fn new() -> MongoDB {
        MongoDB {  
            me_config_mongodb_adminusername: default_username(),
            me_config_mongodb_adminpassword: default_password(),
            me_config_mongodb_url: default_url(),
            me_config_basicauth: default_auth()
        }
    }
    
    pub fn setup_container(&self, docker: Container) -> Container {
        docker
            .add_env_var("ME_CONFIG_MONGODB_ADMINUSERNAME", self.me_config_mongodb_adminusername.clone())
            .add_env_var("ME_CONFIG_MONGODB_ADMINPASSWORD", self.me_config_mongodb_adminpassword.clone())
            .add_env_var("ME_CONFIG_MONGODB_URL", self.me_config_mongodb_url.clone())
            .add_env_var("ME_CONFIG_MONGODB_BASICAUTH", self.me_config_basicauth.to_string())
    }

    pub fn mount_data(&self, schema: Amount<String>, mut docker: Container) -> Container {
        docker
    }
}
