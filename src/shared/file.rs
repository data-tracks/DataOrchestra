use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub name: Option<String>,
    pub path: Option<String>,
    pub destination: Option<String>,
    pub start: Option<String>
}
