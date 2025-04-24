use std::{collections::HashMap, net::{IpAddr, Ipv4Addr}};
use serde::{Deserialize, Serialize};
use crate::types::amount::Amount;


/// The `Docker` type. This struct is used as the overarching struct. It contains the
/// `docker_type`, and the `config` related to it.
/// 
/// The `docker_type` is a docker creation type, which is either a standalone `image`, the path
/// to a `dockerfile` or the path to a docker `compose`. The `config` is then for the additional
/// definining of the container. More over the container parameters can be seen in the [`Container`] type.
///
/// # Example 
///
/// {
///     "image": "ubuntu".
///     "config": 
///     {
///         "name": "ubuntu-container"
///     }
/// }
/// {
///     "dockerfile": "/path/to/dockerfile".
///     "config": 
///     {
///         "name": "ubuntu-container"
///     }
/// }
/// {
///     "compose": "/path/to/compose".
///     "config": 
///     {
///         "name": "ubuntu-container"
///     }
/// }
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Docker {
    #[serde(flatten)]
    pub docker_type: DockerType,    
    pub config: Option<DockerTypeContainer>,
}


/// Docker creation types
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[serde(rename_all = "lowercase")]
pub enum DockerType {
    Image { image: String },
    Dockerfile { dockerfile: String },
    Compose { compose: String }
}

/// The varying [`Container`] types for the different [`DockerType`].
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DockerTypeContainer {
    Default(Container), 
    Compose(Amount<Container>)
}

/// The docker `Container` type. Represents the general information tied to the creation of a
/// docker container
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Container {
    #[serde(default = "default_name")]
    pub name: Option<String>,

    #[serde(default = "default_network")]
    pub network: String,

    pub options: Option<HashMap<String, String>>,

    #[serde(default = "default_mount")]
    pub mount: Option<Amount<String>>,

    #[serde(default = "default_build_args")]
    pub build_args: Option<HashMap<String, String>>,

    #[serde(default = "default_publish_all")]
    pub publish_all: bool,

    /*
     * Creation options
     */
    pub file: Option<String>,
    pub image: Option<String>,
    pub compose: Option<String>,
    
    #[serde(skip)]
    #[serde(default)]
    pub meta: Meta,
}



#[derive(Debug)]
pub struct Meta {
    /// Id of container
    pub id: Option<String>,
    /// Ip of container
    pub ip: Option<IpAddr>,
    /// Published ports of container. A vector of [`PortMap`] which defines the combination
    /// `host:internal`.
    pub publish_ports: Option<Vec<PortMap>>
}

pub fn default_network() -> String {
    String::from("orchestra")
}

pub fn default_mount() -> Option<Amount<String>> {
    None
}

pub fn default_name() -> Option<String> {
    None
}

pub fn default_options() -> Option<HashMap<String, String>> {
    None
}

pub fn default_image() -> Option<String> {
    None
}

pub fn default_compose() -> Option<String> {
    None
}

pub fn default_file() -> Option<String> {
    None
}

pub fn default_build_args() -> Option<HashMap<String, String>> {
    None
}

pub fn default_publish_all() -> bool {
    false
}


impl Default for Meta {
    fn default() -> Self {
        Meta { id: None, ip: None, publish_ports: None }
    }
}

#[derive(Debug)]
pub struct PortMap {
    host: u16,
    internal: u16
}

impl PortMap {
    pub fn new(host: u16, internal: u16) -> Self {
        PortMap { host, internal }
    }

    pub fn get_host(&self) -> u16 {
        self.host.clone()
    }

    pub fn get_internal(&self) -> u16 {
        self.internal.clone()
    }
}
