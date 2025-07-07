use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Executables {
    Script(Script),
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[serde(rename_all = "snake_case")]
pub struct Script {
    #[builder(setter(strip_option, into), default)]
    pub name: Option<String>,
    #[builder(setter(into))]
    pub path: String
}

pub trait GetExecutables {
    fn get_scripts(&self) -> Vec<&Script>;
}

impl GetExecutables for Vec<Executables> {
    fn get_scripts(&self) -> Vec<&Script> {
        let mut vec = Vec::new();

        for item in self {
            match item {
                Executables::Script(script) => vec.push(script),
                _ => ()
            }
        }

        vec
    }
}


