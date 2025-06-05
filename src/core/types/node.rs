use std::net::{IpAddr, Ipv4Addr};

use crate::core::adapters::ssh::Ssh;

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub host: IpAddr,
    pub username: String,
    pub password: Option<String>,
    pub ssh: Option<Ssh>,
    pub ssh_port: u16,
}

impl Node {
    pub fn new() -> Self {
        Node
        {
            name: "Node".to_string(),
            host: IpAddr::V4(Ipv4Addr::LOCALHOST) ,
            username: "root".to_string(),
            password: None,
            ssh: None,
            ssh_port: 22
        }
    }

    pub fn load_ssh(&mut self) -> Result<(), String> {
        let mut ssh = Ssh::new();
        let _ = ssh.connect(&self.host.to_string(), self.ssh_port, &self.username, self.password.as_ref());

        self.ssh = Some(ssh);
        Ok(())
    }
}
