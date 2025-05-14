use std::net::IpAddr;
use serde::{Serialize, Deserialize};
use crate::{core::types::Node, shared::{Address, ToInternal}};


#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all="camelCase")]
pub struct ExtNode {
    pub name: Option<String>,
    pub ip: Option<IpAddr>,
    pub username: Option<String>,
    pub password: Option<String>
}

impl ToInternal<Node> for ExtNode {
    fn to_internal(self) -> Node {
        let mut node = Node::new();

        if let Some(name) = self.name {
            node.name = name;
        }

        if let Some(ip) = self.ip {
            node.ip = ip
        }

        if let Some(user) = self.username {
            node.username = user;
        }

        if let Some(password) = self.password {
            node.password = password;
        }


        node
    }
}
