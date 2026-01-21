use std::{fmt::Display, str::FromStr};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Restart type policies for docker container
#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum RestartTypes {
    No,
    OnFailure(usize),
    Always,
    UnlessStopped
}

impl Default for RestartTypes {
    fn default() -> Self {
        RestartTypes::No
    }
}

impl Display for RestartTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            RestartTypes::No => "no".to_string(), 
            RestartTypes::OnFailure(max_retries) => format!("on-failure[:{max_retries}]"), 
            RestartTypes::Always => "always".to_string(), 
            RestartTypes::UnlessStopped => "unless-stopped".to_string(), 
        };

        write!(f, "{string}")
    }
}

/// Mount bind types for docker container
#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
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

        write!(f, "{string}")
    }
}

/// The container data object. Represents container meta data retrieved through `docker container ls`
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

/// States of a docker container
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

/// The mount object. Represents the mounting configuration of files onto a docker container
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub struct Mount {
    /// Source data location
    pub src: PathBuf,
    /// Destination data location
    pub dst: PathBuf,
    #[serde(default)]
    pub read_only: bool,
    /// Type of bind propagation
    #[serde(default)]
    pub bind_propagation: Option<BindPropagation>
}

impl Mount {
    pub fn new(src: impl Into<PathBuf>, dst: impl Into<PathBuf>, read_only: bool, bind_propagation: Option<BindPropagation>) -> Self {
        Mount { src: src.into(), dst: dst.into(), read_only, bind_propagation }
    }
}
