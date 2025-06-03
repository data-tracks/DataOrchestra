use std::net::{IpAddr, Ipv4Addr};
use serde::{Serialize, Deserialize};
use crate::core::types::Node;
use crate::shared::ToInternal;


#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all="camelCase")]
pub struct ExtNode {
    pub name: Option<String>,
    pub host: IpAddr,
    pub username: Option<String>,
    pub password: Option<String>
}

impl Default for ExtNode {
    fn default() -> Self {
        ExtNode 
        { 
            name: Some("node".to_string()), 
            host: IpAddr::V4(Ipv4Addr::LOCALHOST), 
            username: Some("root".to_string()), 
            password: Some("password".to_string()) 
        }
    }
}

impl ToInternal<Node> for ExtNode {
    fn to_internal(self) -> Node {
        let mut node = Node::new();

        if let Some(name) = self.name {
            node.name = name;
        }

        node.host = node.host;

        if let Some(user) = self.username {
            node.username = user;
        }

        node.password = self.password;

        node
    }
}
