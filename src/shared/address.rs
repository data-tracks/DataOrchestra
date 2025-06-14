use serde::{Deserialize, Serialize};
use std::net::IpAddr;

/// The address object. Represents a remote location with `ip`:`port`
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all="camelCase")]
pub struct Address {
    pub ip: IpAddr,
    pub port: u16
}
