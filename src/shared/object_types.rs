use std::fmt::Display;

use clap::ValueEnum;

use crate::{core::attach::attach_types::AttachTypeConfig, interface::{docker::ExtDocker, generate::ExtGenerate, object::ExtObject, process::ExtProcess, store::ExtStore}};

#[derive(Debug, Clone, ValueEnum)]
pub enum ObjectTypes {
    Store,
    Process,
    Generate,
    Object,
    Docker,
    Attach
}

impl ObjectTypes {
    pub fn print_json(&self) {
        let json: String;

        match self {
            ObjectTypes::Generate => {
                json = serde_json::to_string_pretty(&ExtGenerate::default()).expect("Unable to parse struct to json");
            },
            ObjectTypes::Store => {
                json = serde_json::to_string_pretty(&ExtStore::default()).expect("Unable to parse struct to json");
            },
            ObjectTypes::Process => {
                json = serde_json::to_string_pretty(&ExtProcess::default()).expect("Unable to parse struct to json");
            },
            ObjectTypes::Object => {
                json = serde_json::to_string_pretty(&ExtObject::default()).expect("Unable to parse struct to json");
            }
            ObjectTypes::Docker => {
                json = serde_json::to_string_pretty(&ExtDocker::default()).expect("Unable to parse struct to json");
            }
            ObjectTypes::Attach => {
                json = serde_json::to_string_pretty(&AttachTypeConfig::default()).expect("Unable to parse struct to json");
            }
        }

        println!("{}", json);
    }
}

impl Display for ObjectTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            ObjectTypes::Object => "object".to_string(),
            ObjectTypes::Store => "store".to_string(),
            ObjectTypes::Process => "process".to_string(),
            ObjectTypes::Generate => "generate".to_string(),
            ObjectTypes::Docker => "docker".to_string(),
            ObjectTypes::Attach => "attach".to_string()
        };

        write!(f, "{}", string)
    }
}
