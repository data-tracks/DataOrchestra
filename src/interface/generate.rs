use serde::{Deserialize, Serialize};
use crate::{core::generate::Generate, shared::{traits::ToInternal, Amount}};
use super::config::General;


#[derive(Debug, Deserialize, Serialize)]
pub struct ExtGenerate {
    pub x: String,
    #[serde(default = "default_amount")]
    pub amount: usize,
    #[serde(flatten)]
    pub general: General
}

pub fn default_amount() -> usize {
    1
}

impl ToInternal<Amount<Generate>> for Amount<ExtGenerate> {
    fn to_internal(self) -> Amount<Generate> {
        match self {
            Amount::None => Amount::None,
            Amount::Single(generate) => Amount::Single(generate.to_internal()),
            Amount::Multiple(generates) => {
                let mut vec_generate = Vec::<Generate>::new();
                for generate in generates {
                    vec_generate.push(generate.to_internal());
                };

                Amount::Multiple(vec_generate)
            }
        }
    }
}

impl ToInternal<Generate> for ExtGenerate {
    fn to_internal(self) -> Generate {
        let mut generate = Generate::default();

        generate
    }
}
