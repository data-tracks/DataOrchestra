use std::{collections::HashMap, net::{IpAddr, Ipv4Addr}};
use serde::{Deserialize, Serialize};
use crate::types::{address::Address, amount::Amount};

pub fn default_network() -> String {
    String::from("orchestra")
}

pub fn default_address() -> Address {
    Address {
        ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        port: 5000
    }
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Container {
    #[serde(default = "default_name")]
    pub name: Option<String>,
    #[serde(default = "default_image")]
    pub image: Option<String>,
    #[serde(default = "default_network")]
    pub network: String,
    // Additional options
    pub options: Option<HashMap<String, String>>,
    #[serde(default = "default_address")]
    pub address: Address,
    #[serde(default = "default_mount")]
    pub mount: Option<Amount<String>>,
    #[serde(default = "default_compose")]
    pub compose: Option<String>
}
