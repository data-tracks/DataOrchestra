use std::path::{absolute, Path};

use serde::{Deserialize, Serialize};

use crate::{docker::docker_struct::Container, types::amount::Amount};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "postgres")]
pub struct PostGres {
    #[serde(default = "default_db")]
    pub postgres_db: String,
    #[serde(default = "default_user")]
    pub postgres_user: String,
    #[serde(default = "default_password")]
    pub postgres_password: String,
    #[serde(default = "default_initdb_args")]
    pub postgres_initdb_args: Option<String>
}

pub fn default_db() -> String {
    String::from("postgres")
}

pub fn default_user() -> String {
    String::from("postgres")
}

pub fn default_password() -> String {
    String::from("postgres")
}

pub fn default_initdb_args() -> Option<String> {
    None
}

impl PostGres {
    pub fn new() -> PostGres {
        PostGres 
        { 
            postgres_db: default_db(), 
            postgres_user: default_user(), 
            postgres_password: default_password(),
            postgres_initdb_args: default_initdb_args()
        }
    }

    pub fn setup_container(&self, mut docker: &mut Container)  {
        docker = docker
            .set_image("postgres")
            .add_env_var("POSTGRES_DB", self.postgres_db.clone())
            .add_env_var("POSTGRES_USER", self.postgres_user.clone())
            .add_env_var("POSTGRES_PASSWORD", self.postgres_password.clone());
       
        if let Some(ref initdb_args) = self.postgres_initdb_args {
            docker.add_env_var("POSTGRES_INITDB_ARGS", format!("\"{}\"", initdb_args));
        }
    }

    pub fn mount_data(&self, schema: Amount<String>, mut docker: &mut Container) {
        match schema {
            Amount::None => (),
            Amount::Single(value) => { 
                let _ = docker.add_mount(absolute(Path::new(&value)).unwrap().display().to_string(), format!("/docker-entrypoint-initdb.d/{}", value.clone().split("/").last().unwrap()));
            },
            Amount::Multiple(values) => {
                for value in values {
                    docker = docker.add_mount(absolute(Path::new(&value)).unwrap().display().to_string(), format!("/docker-entrypoint-initdb.d/{}", value.clone().split("/").last().unwrap()));
                }
            }
        };
    }
}
