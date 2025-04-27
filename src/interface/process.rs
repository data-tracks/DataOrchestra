use serde::{Deserialize, Serialize};

use super::config::General;

#[derive(Debug, Deserialize, Serialize)]
pub struct ExtProcess {
    #[serde(default = "default_amount")]
    amount: usize,
    #[serde(flatten)]
    general: General
}

pub fn default_amount() -> usize {
    1
}
