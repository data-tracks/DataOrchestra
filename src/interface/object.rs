use serde::{Deserialize, Serialize};

use super::config::General;

#[derive(Debug, Deserialize, Serialize)]
pub struct ExtObject {
    #[serde(default = "default_amount")]
    pub amount: usize,
    #[serde(flatten)]
    pub general: General
}

pub fn default_amount() -> usize {
    1
}


