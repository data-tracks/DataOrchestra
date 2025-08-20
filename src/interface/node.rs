use std::net::{IpAddr, Ipv4Addr};
use serde::{Serialize, Deserialize};
use crate::core::adapters::Uploader;
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
    upload_schema: UploadTypes
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

impl ToInternal<Node> for ExtNode {
    fn to_internal(self) -> Node {
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

        node
    }

}
