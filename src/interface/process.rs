use serde::{Deserialize, Serialize};

use crate::{core::process::Process, shared::{traits::ToInternal, Amount}};

use super::config::General;

#[derive(Debug, Deserialize, Serialize)]
pub struct ExtProcess {
    #[serde(default = "default_amount")]
    pub amount: usize,
    #[serde(flatten)]
    pub general: General
}

pub fn default_amount() -> usize {
    1
}

impl ToInternal<Amount<Process>> for Amount<ExtProcess> {
    fn to_internal(self) -> Amount<Process> {
        match self {
            Amount::None => Amount::None,
            Amount::Single(process) => Amount::Single(process.to_internal()),
            Amount::Multiple(processes) => {
                let mut vec_process = Vec::<Process>::new();
                for process in processes {
                    vec_process.push(process.to_internal());
                };

                Amount::Multiple(vec_process)
            }
        }
    }
}

impl ToInternal<Process> for ExtProcess {
    fn to_internal(self) -> Process {
        let mut process = Process::default();

        process
    }
}
