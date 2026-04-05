use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum Executables {
    Script(Script),
}

#[derive(Debug, Clone, Builder)]
pub struct Script {
    #[builder(setter(strip_option, into), default)]
    pub name: Option<String>,
    #[builder(setter(into))]
    pub path: String
}

impl Executables {
    pub fn get_script(&self) -> &Script {
        match self {
            Executables::Script(script) => script,
            _ => panic!("Get script for non-script")
        }
    }
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


