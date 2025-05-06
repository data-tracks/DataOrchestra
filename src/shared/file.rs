use serde::{Deserialize, Serialize};
use crate::core::data::Data;
use super::ToInternal;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct File {
    pub name: Option<String>,
    pub path: Option<String>,
    pub destination: Option<String>,
    pub start: Option<String>,
    pub dependency: Option<String>
}

impl ToInternal<Data> for File {
    fn to_internal(self) -> Data {
        let mut data = Data::default();

        if let Some(name) = self.name {
            data.name = name;
        }
        if let Some(path) = self.path {
            data.path = path;
        }
        if let Some(destination) = self.destination {
            data.destination = destination;
        }
        if let Some(start) = self.start {
            data.start = start;
        }

        data.dependency = self.dependency;

        data
    }
}
