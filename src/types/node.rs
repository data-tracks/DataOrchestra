use serde::{Serialize, Deserialize};
use super::address::Address;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Node {
    pub name: Option<String>,
    pub address: Option<Address>
}
