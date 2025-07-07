use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Location {
    Container,
    Node
}

impl Default for Location {
    fn default() -> Self {
        Location::Container
    }
}
