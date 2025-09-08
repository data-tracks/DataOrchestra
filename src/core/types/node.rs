use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;
use log::error;

use crate::core::adapters::ssh::Ssh;

/// Node object. Represents a remote host and its connection options to this remote node via ssh
#[derive(Debug)]
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
    pub ssh_key: Option<PathBuf>,
    /// Ssh object 
    pub ssh: Option<Ssh>,
    /// Ssh port
    pub ssh_port: u16,
}

/// Clone for Node. Clones all fields but creates a new ssh session due to the SSH channel blocking.
impl Clone for Node {
    fn clone(&self) -> Self {
        let mut ssh: Option<Ssh> = None;
        if self.ssh.is_some() {
            let mut new_ssh = Ssh::new();
            let result = new_ssh.connect_ssh(&self.host.to_string(), self.ssh_port, &self.username, self.ssh_key.as_ref().unwrap());
            if let Err(error) = result {
                error!("{error}");
            }
            ssh = Some(new_ssh);

        }
        Node {
            name: self.name.clone(),
            host: self.host,
            username: self.username.clone(),
            password: self.password.clone(),
            ssh_key: self.ssh_key.clone(),
            ssh, 
            ssh_port: self.ssh_port,
        }
    }
}

impl Default for Node {
    fn default() -> Self {
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
}

impl Node {
    /// Load ssh session from node and set ssh field in node object
    pub fn set_ssh(&mut self) -> Result<(), String> {
        let ssh = self.get_ssh();
        self.ssh = Some(ssh);
        Ok(())
    }

    /// Get new ssh session from node
    pub fn get_ssh(&self) -> Ssh {
        let mut ssh = Ssh::new();
        let _ = ssh.connect_ssh(&self.host.to_string(), self.ssh_port, &self.username, self.ssh_key.as_ref().unwrap());

        ssh
    }
}
