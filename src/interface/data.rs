use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::core::adapters::Mount;
use crate::core::types::data::{Data, DataTypes, VolatileData};
use crate::shared::traits::ToInternal;

use super::location::Location;

/// Data types. Represents different types of data which can be uploaded.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ExtDataTypes {
    Volatile(ExtVolatile),
    Data(ExtData)
}

impl ExtDataTypes {
    pub fn is_volatile(&self) -> bool {
        matches!(self, ExtDataTypes::Volatile(_))
    }

    pub fn is_data(&self) -> bool {
        matches!(self, ExtDataTypes::Data(_))
    }

    pub fn get_volatile_ref(&self) -> &ExtVolatile {
        match self {
            ExtDataTypes::Volatile(volatile) => volatile,
            _ => panic!("Get volatile on non-volatile")
        }
    }

    pub fn get_data_ref(&self) -> &ExtData {
        match self {
            ExtDataTypes::Data(data) => data,
            _ => panic!("Get data on non-data")
        }
    }

    pub fn get_volatile_mut(&mut self) -> &mut ExtVolatile {
        match self {
            ExtDataTypes::Volatile(volatile) => volatile,
            _ => panic!("Get volatile on non-volatile")
        }
    }

    pub fn get_data_mut(&mut self) -> &mut ExtData {
        match self {
            ExtDataTypes::Data(data) => data,
            _ => panic!("Get data on non-data")
        }
    }
}

/// Data type. Represents existing data which is uploaded to a remote location
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct ExtData {
    #[serde(default)]
    pub location: Location,
    #[serde(default)]
    pub name: Option<String>,
    pub source: String,
    pub destination: String,
    #[serde(default)]
    pub dependency: Option<String>,
}

impl ToInternal<(DataTypes, Option<Mount>)> for ExtData {
    fn to_internal(self) -> (DataTypes, Option<Mount>) {
        match self.location {
            Location::Node => {
                let data = Data
                {
                    name: self.name,
                    src: self.source.clone().into(),
                    dst: self.destination.clone().into(),
                    dependency: self.dependency
                };

                (DataTypes::Data(data), None)
            },
            Location::Container => {
                let remote_location = PathBuf::from(format!("docker/mount/data/{}", self.destination.clone()));

                let data = Data
                {
                    name: self.name,
                    src: self.source.clone().into(),
                    dst: remote_location.clone(),
                    dependency: self.dependency
                };

                let mount = Mount
                {
                    src: remote_location,
                    dst: self.destination.into(),
                    read_only: false,
                    bind_propagation: None,
                };
                (DataTypes::Data(data), Some(mount))
            }
        }
    }
}

/// Volatile type. Represents non-existing data, where the content of `volatile_types` is written to a file on the remote location
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ExtVolatile {
    #[serde(default)]
    pub location: Location,
    pub name: Option<String>,
    pub destination: String,
    #[serde(flatten)]
    pub volatile_types: VolatileTypes
}

impl ToInternal<(DataTypes, Option<Mount>)> for ExtVolatile {
    fn to_internal(self) -> (DataTypes, Option<Mount>) {


        match self.location {
            Location::Node => {
                let data = VolatileData
                {
                    name: self.name,
                    dst: self.destination.clone().into(),
                    content: self.volatile_types.to_string()
                };
                
                (DataTypes::VolatileData(data), None)
            },
            Location::Container => {
                let remote_location = PathBuf::from(format!("docker/mount/volatile/{}", self.destination.clone()));

                let data = VolatileData
                {
                    name: self.name,
                    dst: remote_location.clone().into(),
                    content: self.volatile_types.to_string()
                };
                
                let mount = Mount
                {
                    src: remote_location,
                    dst: self.destination.into(),
                    read_only: false,
                    bind_propagation: None,
                };

                (DataTypes::VolatileData(data), Some(mount))
            }
        }
    }
}

impl ToInternal<(DataTypes, Option<Mount>)> for ExtDataTypes {
    fn to_internal(self) -> (DataTypes, Option<Mount>) {
        match self {
            ExtDataTypes::Data(data) => data.to_internal(),
            ExtDataTypes::Volatile(volatile_data) => volatile_data.to_internal(),
        }
    }
}

/// Types of structured volatile data.
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum VolatileTypes {
    Content(String),
    Json(Map<String, Value>),
    Env(HashMap<String, String>),
}


impl Display for VolatileTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VolatileTypes::Content(content) => write!(f, "{content}"),
            VolatileTypes::Env(env) => {
                let mut string = String::new();
                for (key, value) in env.iter() {
                    string.push_str(format!("{}={}\n", key, value).as_str())
                }
                string.pop();

                write!(f, "{string}")
            },
            VolatileTypes::Json(json) => {
                let string = serde_json::to_string_pretty(&json).expect("Unable to parse volatile json to string");
                write!(f, "{string}")
            }
        }
    }
}
