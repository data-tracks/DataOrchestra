use serde::{Deserialize, Serialize};

use crate::core::adapters::docker::container::ContainerBuilder;

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

    pub fn setup_container(&self, mut docker: &mut ContainerBuilder)  {
        docker = docker
            .try_set_name("postgres")
            .set_image("postgres")
            .add_env_var("POSTGRES_DB", self.postgres_db.clone())
            .add_env_var("POSTGRES_USER", self.postgres_user.clone())
            .add_env_var("POSTGRES_PASSWORD", self.postgres_password.clone())
            .add_publish(5432);

        if let Some(ref initdb_args) = self.postgres_initdb_args {
            docker.add_env_var("POSTGRES_INITDB_ARGS", format!("\"{}\"", initdb_args));
        }
    }

    pub fn mount_data(&self, mounts: &Vec<String>, mut docker: &mut ContainerBuilder) {
        for mount in mounts {
            docker = docker.add_mount(format!("{}:{}", &mount, format!("/docker-entrypoint-initdb.d/{}", mount.clone().split("/").last().unwrap())));
        }
    }
}
