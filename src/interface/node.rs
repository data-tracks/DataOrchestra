use std::net::IpAddr;
use serde::{Serialize, Deserialize};
use crate::core::types::Node;
use crate::shared::ToInternal;


#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all="camelCase")]
pub struct ExtNode {
    pub name: Option<String>,
    pub host: Option<IpAddr>,
    pub username: Option<String>,
    pub password: Option<String>
}

impl ToInternal<Node> for ExtNode {
    fn to_internal(self) -> Node {
        let mut node = Node::new();

        if let Some(name) = self.name {
            node.name = name;
        }

        if let Some(ip) = self.host {
            node.host = ip
        }

        if let Some(user) = self.username {
            node.username = user;
        }

        node.password = self.password;

        node
    }
}
