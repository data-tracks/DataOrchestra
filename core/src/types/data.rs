use std::path::PathBuf;

use derive_builder::Builder;

#[derive(Debug, Clone)]
pub enum DataTypes {
    Data(Data),
    VolatileData(VolatileData),
}

impl DataTypes {
    pub fn is_data(&self) -> bool {
        matches!(self, DataTypes::Data(_))
    }

    pub fn is_volatile_data(&self) -> bool {
        matches!(self, DataTypes::VolatileData(_))
    }

    pub fn get_data_ref(&self) -> &Data {
        match self {
            DataTypes::Data(data) => data,
            _ => panic!("Get data from non data datatype"),
        }
    }

    pub fn get_data_mut(&mut self) -> &mut Data {
        match self {
            DataTypes::Data(data) => data,
            _ => panic!("Get data from non data datatype"),
        }
    }

    pub fn get_volatile_data_ref(&self) -> &VolatileData {
        match self {
            DataTypes::VolatileData(volatile) => volatile,
            _ => panic!("Get data from non data datatype"),
        }
    }

    pub fn get_volatile_data_mut(&mut self) -> &mut VolatileData {
        match self {
            DataTypes::VolatileData(volatile) => volatile,
            _ => panic!("Get data from non data datatype"),
        }
    }
}

impl GetVecData for Vec<DataTypes> {
    fn get_volatile_data_ref(&self) -> Vec<&VolatileData> {
        self.iter()
            .filter(|item| item.is_volatile_data())
            .map(|item| item.get_volatile_data_ref())
            .collect()
    }

    fn get_volatile_data_mut(&mut self) -> Vec<&mut VolatileData> {
        self.iter_mut()
            .filter(|item| item.is_volatile_data())
            .map(|item| item.get_volatile_data_mut())
            .collect()
    }

    fn get_data_ref(&self) -> Vec<&Data> {
        self.iter()
            .filter(|item| item.is_data())
            .map(|item| item.get_data_ref())
            .collect()
    }

    fn get_data_mut(&mut self) -> Vec<&mut Data> {
        self.iter_mut()
            .filter(|item| item.is_data())
            .map(|item| item.get_data_mut())
            .collect()
    }
}

pub trait GetVecData {
    fn get_volatile_data_ref(&self) -> Vec<&VolatileData>;
    fn get_volatile_data_mut(&mut self) -> Vec<&mut VolatileData>;
    fn get_data_ref(&self) -> Vec<&Data>;
    fn get_data_mut(&mut self) -> Vec<&mut Data>;
}

#[derive(Debug, Clone, Builder)]
pub struct VolatileData {
    /// Name of docker container
    #[builder(setter(strip_option, into), default)]
    pub name: Option<String>,
    /// File name
    #[builder(setter(into))]
    pub dst: PathBuf,
    /// Data written into file
    #[builder(setter(into))]
    pub content: String,
}

#[derive(Debug, Clone, Builder)]
pub struct Data {
    /// Name of container
    #[builder(setter(strip_option, into), default)]
    pub name: Option<String>,
    /// Path of data in current system
    #[builder(setter(into))]
    pub src: PathBuf,
    /// Path of data in docker container
    #[builder(setter(into))]
    pub dst: PathBuf,
    #[builder(setter(strip_option, into))]
    #[builder(default)]
    pub dependency: Option<String>,
}
