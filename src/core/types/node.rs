use std::net::{IpAddr, Ipv4Addr};

use crate::core::adapters::ssh::Ssh;

/// Node object. Represents a remote host and its connection options to this remote node via ssh
#[derive(Debug, Clone)]
pub struct Node {
    /// Node name
    pub name: String,
    /// Node host ip
    pub host: IpAddr,
    /// Ssh username
    pub username: String,
    /// Ssh password
    pub password: Option<String>,
    /// Path to ssh key
    pub ssh_key: Option<String>,
    /// Ssh object 
    pub ssh: Option<Ssh>,
    /// Ssh port
    pub ssh_port: u16,
}

impl Node {
    pub fn new() -> Self {
        Node
        {
            name: "Node".to_string(),
            host: IpAddr::V4(Ipv4Addr::LOCALHOST),
            username: "root".to_string(),
            password: None,
            ssh_key: None,
            ssh: None,
            ssh_port: 22
        }
    }

    /// Load ssh session from node and set ssh field in node object
    pub fn set_ssh(&mut self, ssh_key: &String) -> Result<(), String> {
        let ssh = self.get_ssh(ssh_key);
        self.ssh = Some(ssh);
        Ok(())
    }

    /// Get new ssh session from node
    pub fn get_ssh(&self, ssh_key: &String) -> Ssh {
        let mut ssh = Ssh::new();
        let _ = ssh.connect_ssh(&self.host.to_string(), self.ssh_port, &self.username, ssh_key);

        ssh
    }
}
