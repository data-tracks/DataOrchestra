use serde::{Deserialize, Serialize};
use crate::docker::docker_struct::Container;


pub trait StoreData {
    fn get_image(&self) -> String;
    fn setup_container(&self, docker: &mut Container) -> Container;
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StoreType {
    PostGres,
    Redis,
    MySQL,
    MongoDB,
    Polypheny
    // clustering?
}

/// Represents the available storing types
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum StoreTypeConfig {
    PostGres(PostGres),
    Redis(Redis),
    MySQL(MySQL),
    MongoDB(MongoDB),
    Polypheny(Polypheny)
}

impl StoreData for StoreTypeConfig {
    fn get_image(&self) -> String {
        match self {
            StoreTypeConfig::PostGres(postgres) => postgres.get_image(),
            StoreTypeConfig::Redis(redis) => redis.get_image(),
            StoreTypeConfig::MySQL(mysql) => mysql.get_image(),
            StoreTypeConfig::MongoDB(mongodb) => mongodb.get_image(),
            StoreTypeConfig::Polypheny(polypheny) => polypheny.get_image()
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "postgres")]
pub struct PostGres {
    #[serde(default = "default_db")]
    pub postgres_db: String,
    #[serde(default = "default_user")]
    pub postgres_user: String,
    #[serde(default = "default_password")]
    pub postgres_password: String,
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

impl PostGres {
    pub fn new() -> PostGres {
        PostGres { postgres_db: default_db(), postgres_user: default_user(), postgres_password: default_password() }
    }
}

impl StoreData for PostGres {
    fn get_image(&self) -> String {
        String::from("postgres")
    }
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "redis")]
pub struct Redis {
}

impl Redis {
    pub fn new() -> Redis {
        Redis {  }
    }
}

impl StoreData for Redis {
    fn get_image(&self) -> String {
        String::from("redis")
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "mysql")]
pub struct MySQL {}

impl MySQL {
    pub fn new() -> MySQL {
        MySQL {  }
    }
}

impl StoreData for MySQL {
    fn get_image(&self) -> String {
        String::from("mysql")
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "mongodb")]
pub struct MongoDB {}

impl MongoDB {
    pub fn new() -> MongoDB {
        MongoDB {  }
    }
}

impl StoreData for MongoDB {
    fn get_image(&self) -> String {
        String::from("mongo")
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "polypheny")]
pub struct Polypheny {}

impl Polypheny {
    pub fn new() -> Polypheny {
        Polypheny {  }
    }
}

impl StoreData for Polypheny {
    fn get_image(&self) -> String {
        String::from("polypheny")
    }
}
