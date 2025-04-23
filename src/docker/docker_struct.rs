use std::{collections::HashMap, net::{IpAddr, Ipv4Addr}};
use serde::{Deserialize, Serialize};
use crate::types::amount::Amount;

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


/// The docker `Container` type. Represents the general information tied to the creation of a
/// docker container
///
/// # Creation
///
/// The creation of the container is dictated by the fields [`image`], [`compose`] and [`file`]. 
/// They are in the order hierarchy: [`compose`] > [`file`] > [`image`], meaning that if both a
/// compose and image are passed, the compose is preferred over the image.
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
    
    

    /*
     * Container values
     */

    #[serde(skip)]
    #[serde(default)]
    pub meta: Meta,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Docker {
    #[serde(flatten)]
    pub docker_type: DockerType,
    pub config: Option<DockerTypeContainer>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DockerType {
    Image { image: String },
    Dockerfile { dockerfile: String },
    Compose { compose: String }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DockerTypeContainer {
    Default(Container), 
    Compose(Vec<Container>)
}


#[derive(Debug)]
pub struct Meta {
    pub id: Option<String>,
    pub ip: Option<IpAddr>,
    pub publish_ports: Option<Vec<PortMap>>
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
