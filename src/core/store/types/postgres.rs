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
    pub postgres_initdb_args: Option<String>,
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

    /// Setup postgres container with default data
    pub fn setup_container(&self, mut docker: &mut ContainerBuilder)  {
        docker = docker
            .try_name_mut("postgres")
            .image_mut("postgres")
            .env_var_mut("POSTGRES_DB", self.postgres_db.clone())
            .env_var_mut("POSTGRES_USER", self.postgres_user.clone())
            .env_var_mut("POSTGRES_PASSWORD", self.postgres_password.clone())
            .publish_mut(5432);

        if let Some(initdb_args) = self.postgres_initdb_args.as_ref() {
            docker.env_var_mut("POSTGRES_INITDB_ARGS", format!("\"{}\"", initdb_args));
        }
    }

    /// Mount data to postgres database
    pub fn mount_data(&self, mounts: &Vec<String>, mut docker: &mut ContainerBuilder) {
        for mount in mounts {
            // TODO: Conditionally check if user already provides full path
            // Mount requires full path, $(pwd) inserts the needed base directory
            docker = docker.volume_mut(format!("$(pwd)/{}:{}", &mount, format!("/docker-entrypoint-initdb.d/{}", mount.clone().split("/").last().unwrap())));
        }
    }

    /// Get connection string for postgres DB
    pub fn get_connection_string(&self, host: String, port: u16) -> String {
        format!("host={} port=5432 user={} password={}", host, port, self.postgres_password)
    }
}
