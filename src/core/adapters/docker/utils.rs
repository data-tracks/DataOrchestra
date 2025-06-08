use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize, Hash, PartialEq, Eq)]
pub enum BindPropagation {
    Shared,
    Slave,
    Private,
    RShared,
    RSlave,
    RPrivate
}

impl Display for BindPropagation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            BindPropagation::Slave => "slave",
            BindPropagation::Shared => "shared",
            BindPropagation::RSlave => "rslave",
            BindPropagation::Private => "private",
            BindPropagation::RShared => "rshared",
            BindPropagation::RPrivate => "rprivate"
        };

        write!(f, "{}", string)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename = "snake_case")]
pub struct ContainerData {
    pub command: String,
    pub created_at: String,
    pub id: String,
    pub image: String,
    pub labels: String,
    pub local_volumes: usize,
    pub mounts: String,
    pub names: String,
    pub networks: String,
    pub ports: String,
    pub running_for: String,
    pub size: String,
    pub state: StateTypes,
    pub status: String
}

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq)]
pub enum StateTypes {
    Created,
    Running,
    Complete,
    Failed,
    Die
}


