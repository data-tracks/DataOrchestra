use serde::{Deserialize, Serialize};

use crate::core::{store::Store, traits::{Checkable, Configurator}};

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

impl Configurator<Store> for PostGres {
    fn configure(&mut self, parent: &mut Store) {

        let container = parent.object.docker_container_builder.get_or_insert_default(); 
        container
            .try_name("postgres")
            .image("postgres")
            .ignore_ssh(true)
            .environment("POSTGRES_DB", self.postgres_db.clone())
            .environment("POSTGRES_USER", self.postgres_user.clone())
            .environment("POSTGRES_PASSWORD", self.postgres_password.clone())
            .publish(5432);

        if let Some(initdb_args) = self.postgres_initdb_args.as_ref() {
            container.environment("POSTGRES_INITDB_ARGS", format!("\"{}\"", initdb_args));
        }

        // Mount sql schemas to container
        for mount in parent.schema.iter() {
            // TODO: Conditionally check if user already provides full path
            // Mount requires full path, $(pwd) inserts the needed base directory
            container.volume(format!("$(pwd)/{}:{}", &mount, format!("/docker-entrypoint-initdb.d/{}", mount.clone().split("/").last().unwrap())));
        }
    }
}

impl Default for PostGres {
    fn default() -> PostGres {
        PostGres 
        { 
            postgres_db: default_db(), 
            postgres_user: default_user(), 
            postgres_password: default_password(),
            postgres_initdb_args: default_initdb_args()
        }
    }
}

impl PostGres {
    /// Get connection string for postgres DB
    pub fn get_connection_string(&self, host: String, port: u16) -> String {
        format!("host={} port=5432 user={} password={}", host, port, self.postgres_password)
    }
}

impl Checkable<()> for PostGres {
    fn check(&self) -> Result<(), String> {
        Ok(()) 
    }
}
