use serde::{Serialize, Deserialize};
use crate::core::types::Node;

use super::{address::Address, ToInternal};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all="camelCase")]
pub struct ExtNode {
    pub name: Option<String>,
    pub address: Option<Address>,
    pub user: Option<String>,
    pub password: Option<String>
}

impl ToInternal<Node> for ExtNode {
    fn to_internal(self) -> Node {
        let mut node = Node::new();

        if let Some(name) = self.name {
            node.name = name;
        }

        if let Some(address) = self.address {
            node.address = address
        }

        if let Some(user) = self.user {
            node.user = user;
        }

        if let Some(password) = self.password {
            node.password = password;
        }


        node
    }
}
