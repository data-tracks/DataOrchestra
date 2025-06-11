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

    pub fn set_ssh(&mut self, ssh_key: &String) -> Result<(), String> {
        let ssh = self.get_ssh(ssh_key);
        self.ssh = Some(ssh);
        Ok(())
    }

    pub fn get_ssh(&self, ssh_key: &String) -> Ssh {
        let mut ssh = Ssh::new();
        let _ = ssh.connect_ssh(&self.host.to_string(), self.ssh_port, &self.username, ssh_key);

        ssh
    }
}
