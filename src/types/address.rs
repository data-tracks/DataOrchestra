use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all="camelCase")]
pub struct Address {
    pub ip: IpAddr,
    pub port: u16,
    pub internal_port: u8
}
