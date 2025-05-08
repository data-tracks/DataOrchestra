use std::net::{IpAddr, Ipv4Addr};

use crate::shared::address::Address;
use crate::core::adapters::ssh::Ssh;

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub address: Address,
    pub user: String,
    pub password: String,
    pub ssh: Option<Ssh>
}

impl Node {
    pub fn new() -> Self {
        Node
        {
            name: "Node".to_string(),
            address: Address { ip: IpAddr::V4(Ipv4Addr::LOCALHOST), port: 5000 },
            user: "root".to_string(),
            password: "password".to_string(),
            ssh: None
        }
    }
}
