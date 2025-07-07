use serde::{Deserialize, Serialize};
use crate::core::types::data::{Data, DataTypes, VolatileData};
use crate::shared::traits::ToInternal;

use super::location::Location;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ExtDataTypes {
    Volatile(ExtVolatile),
    Data(ExtData)
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExtData {
    #[serde(default)]
    pub location: Location,
    #[serde(default)]
    pub name: Option<String>,
    pub path: String,
    pub destination: String,
    #[serde(default)]
    pub dependency: Option<String>,
}

impl ToInternal<DataTypes> for ExtData {
    fn to_internal(self) -> DataTypes {
        let data = Data 
        {
            name: self.name,
            path: self.path,
            destination: self.destination,
            dependency: self.dependency
        };

        match self.location {
            Location::Node => DataTypes::NodeData(data),
            Location::Container => DataTypes::DockerData(data)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExtVolatile {
    #[serde(default)]
    pub location: Location,
    pub name: Option<String>,
    pub content: String,
    pub destination: String
}

impl ToInternal<DataTypes> for ExtVolatile {
    fn to_internal(self) -> DataTypes {
        let data = VolatileData 
        {
            name: self.name,
            file: self.destination.into(),
            content: self.content
        };

        match self.location {
            Location::Node => DataTypes::VolatileNodeData(data),
            Location::Container => DataTypes::VolatileDockerData(data)
        }
    }
}

impl ToInternal<DataTypes> for ExtDataTypes {
    fn to_internal(self) -> DataTypes {
        match self {
            ExtDataTypes::Data(data) => data.to_internal(),
            ExtDataTypes::Volatile(volatile_data) => volatile_data.to_internal(),
        }
    }
}
/*

/// External representation of the internal [`NodeData`] object 
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExtNodeData {
    pub path: String,
    pub destination: Option<String>
}



#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExtTmux {
    #[serde(default)]
    session: Option<String>,
    #[serde(default)]
    command: Amount<String>
}

impl Default for ExtNodeData {
    fn default() -> Self {
        ExtNodeData 
        { 
            path: "".to_string(), 
            destination: Some("".to_string())
        }
    }
}

impl ToInternal<NodeData> for ExtNodeData {
    fn to_internal(self) -> NodeData {
        let mut data = NodeData::default();

        data.path = self.path;

        if let Some(destination) = self.destination {
            data.destination = destination;
        }

        data
    }
}

/// External representation of the internal [`DockerData`] object
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExtDockerData {
    pub name: Option<String>,
    pub path: String,
    pub destination: Option<String>,
    pub env: Option<HashMap<String, String>>,
    pub start: Option<String>,
    pub dependency: Option<String>,
    pub tmux: Option<ExtTmux>
}

impl Default for ExtDockerData {
    fn default() -> Self {
        ExtDockerData 
        { 
            name: None, 
            path: "".to_string(), 
            destination: None, 
            env: None,
            start: None, 
            dependency: None,
            tmux: None
        }
    }
}

impl ToInternal<DockerData> for ExtDockerData {
    fn to_internal(self) -> DockerData {
        let mut data = DockerData::default();

        data.name = self.name;

        data.path = self.path;

        if let Some(destination) = self.destination {
            if !destination.starts_with("/") {
                error!("File path doesn't start with /<path>");
            }
            data.destination = destination;
        }
        if let Some(start) = self.start {
            data.start = start;
        }

        data.dependency = self.dependency;

        data
    }
}

/// External representation of the internal [`VolatileDockerData`] object
#[derive(Debug, Deserialize, Serialize)]
pub struct ExtVolatileDockerData {
    pub name: Option<String>,
    pub file: PathBuf,
    pub data: Value
}

impl Default for ExtVolatileDockerData {
    fn default() -> Self {
        ExtVolatileDockerData 
        { 
            name: None, 
            file: PathBuf::default(), 
            data: Value::Null 
        }
    }
}

impl ToInternal<VolatileDockerData> for ExtVolatileDockerData {
    fn to_internal(self) -> VolatileDockerData {
        let data = serde_json::to_string_pretty(&self.data).expect("Unable to parse data to string");
        VolatileDockerData::new(self.name, self.file, data) 
    }
}
*/
