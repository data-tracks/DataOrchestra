use serde::{Deserialize, Serialize};

use crate::shared::Amount;

use super::config::{General, File};

#[derive(Debug, Deserialize, Serialize)]
pub struct ExternalObject {
    #[serde(default = "default_amount")]
    amount: usize,
    #[serde(flatten)]
    general: General,
    files: Amount<File>
}

pub fn default_amount() -> usize {
    1
}


