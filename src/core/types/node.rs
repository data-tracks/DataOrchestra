use std::net::{IpAddr, Ipv4Addr};

use crate::core::adapters::Executor;
use crate::shared::address::Address;
use crate::core::adapters::ssh::Ssh;

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub address: Address,
    pub username: String,
    pub password: String,
    pub ssh: Option<Ssh>
}

impl Node {
    pub fn new() -> Self {
        Node
        {
            name: "Node".to_string(),
            address: Address { ip: IpAddr::V4(Ipv4Addr::LOCALHOST), port: 5000 },
            username: "root".to_string(),
            password: "password".to_string(),
            ssh: None
        }
    }

    pub fn load_ssh(&mut self) -> Result<(), String> {
        let mut ssh = Ssh::new();
        let _ = ssh.connect(&self.address.ip.to_string(), self.get_ssh_port().unwrap(), &self.username, &self.password);

        self.ssh = Some(ssh);
        Ok(())
    }

    pub fn get_ssh_port(&self) -> Option<u16> {
        Some(1)
    }
}
