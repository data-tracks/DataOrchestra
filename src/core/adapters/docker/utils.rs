use std::{fmt::Display, str::FromStr};

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
#[serde(rename_all = "PascalCase")]
pub struct ContainerData {
    pub created_at: String,
    #[serde(rename = "ID")]
    pub id: String,
    pub image: String,
    pub labels: String,
    pub local_volumes: String,
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
#[serde(rename_all = "lowercase")]
pub enum StateTypes {
    Created,
    Running,
    Complete,
    Failed,
    Die,
    Exited
}

impl FromStr for StateTypes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("created") {
            return Ok(StateTypes::Created);
        }
        else if s.contains("running") {
            return Ok(StateTypes::Running);
        }
        else if s.contains("complete") {
            return Ok(StateTypes::Complete);
        }
        else if s.contains("failed") {
            return Ok(StateTypes::Failed);
        }
        else if s.contains("die") {
            return Ok(StateTypes::Die);
        }
        else if s.contains("exited") {
            return Ok(StateTypes::Exited);
        }

        Err("State doesnt exist".to_string())
    }
}


