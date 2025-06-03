use std::fmt::Display;

use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum ObjectTypes {
    Store,
    Process,
    Generate,
    Object
}

impl Display for ObjectTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            ObjectTypes::Object => "object".to_string(),
            ObjectTypes::Store => "store".to_string(),
            ObjectTypes::Process => "process".to_string(),
            ObjectTypes::Generate => "generate".to_string()
        };

        write!(f, "{}", string)
    }
}
