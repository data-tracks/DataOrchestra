use serde::{Deserialize, Serialize};

use crate::core::adapters::docker::container::ContainerBuilder;

pub fn default_username() -> String {
    String::from("mongo")
}

pub fn default_password() -> String {
    String::from("mongo")
}

/// The `MongoDB` type. Represents the configurability of the MongoDB instance
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "mongodb")]
pub struct MongoDB {
    #[serde(default = "default_username")]
    username: String,
    #[serde(default = "default_password")]
    password: String,
}

impl MongoDB {
    pub fn new() -> MongoDB {
        MongoDB {  
            username: default_username(),
            password: default_password()
        }
    }
   
    /// Sets up the docker [`ContainerBuilder`] with the configuration specific to the MongoDB
    /// application
    pub fn setup_container(&mut self, docker: &mut ContainerBuilder) {
        docker.set_image("mongo:4.4.6");
        docker
            .try_set_name("mongodb")
            .add_publish_map(27017, 27017)
            .add_env_var("MONGO_INITDB_ROOT_USERNAME", self.username.clone())
            .add_env_var("MONGO_INITDB_ROOT_PASSWORD", self.password.clone());
    }

    pub fn mount_data(&self, mounts: &Vec<String>, mut docker: &mut ContainerBuilder) {
        for mount in mounts {
            docker = docker.add_mount(format!("{}:{}", &mount, format!("/docker-entrypoint-initdb.d/{}", mount.clone().split("/").last().unwrap())));
        }
    }
}
