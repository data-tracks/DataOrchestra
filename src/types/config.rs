use serde::{Serialize, Deserialize};

use crate::store::store_struct::Store;
use crate::process::process_struct::Process;
use crate::generate::generate_struct::Generate;

use super::amount::Amount;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Config {
    pub process: Amount<Process>,
    pub generate: Amount<Generate>,
    pub store: Amount<Store>
}
