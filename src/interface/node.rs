use std::net::{IpAddr, Ipv4Addr};
use serde::{Serialize, Deserialize};
use crate::core::adapters::{Ssh, Uploader};
use crate::core::types::Node;
use crate::interface::upload::UploadTypes;
use crate::shared::ToInternal;

/// External representation of the internal [`Node`] object
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all="snake_case")]
pub struct ExtNode {
    pub name: Option<String>,
    pub host: IpAddr,
    pub username: Option<String>,
    pub password: Option<String>,
    #[serde(default = "default_ssh_port")]
    pub ssh_port: u16,
    #[serde(default)]
    pub upload_schema: UploadTypes
}



pub fn default_ssh_port() -> u16 {
    22
}

impl Default for ExtNode {
    fn default() -> Self {
        ExtNode 
        { 
            name: Some("node".to_string()), 
            host: IpAddr::V4(Ipv4Addr::LOCALHOST), 
            username: Some("root".to_string()), 
            password: Some("password".to_string()),
            ssh_port: default_ssh_port(),
            upload_schema: UploadTypes::default()
        }
    }
}

impl ToInternal<(Node, Box<dyn Uploader + Send + Sync>)> for ExtNode {
    fn to_internal(self) -> (Node, Box<dyn Uploader + Send + Sync>) {
        let mut node = Node::default();

        if let Some(name) = self.name {
            node.name = name;
        }

        node.host = self.host;
        node.ssh_port = self.ssh_port;

        if let Some(user) = self.username {
            node.username = user;
        }

        node.password = self.password;

        let uploader: Box<dyn Uploader + Send + Sync>;
        match self.upload_schema {
            UploadTypes::Ssh => {
                uploader = Ssh::new().to_box_uploader();
            },
            UploadTypes::Rsync(rsync) => {
                let rsync = rsync.to_internal();
                uploader = rsync.to_box_uploader();
            }
        }

        (node, uploader)
    }

}
