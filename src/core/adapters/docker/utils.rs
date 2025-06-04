use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize, Hash, PartialEq, Eq)]
pub enum BindPropagation {
    Shared,
    Slave,
    Private,
    RShared,
    RSlave,
    RPrivate
}

impl Display for BindPropagation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            BindPropagation::Slave => "slave",
            BindPropagation::Shared => "shared",
            BindPropagation::RSlave => "rslave",
            BindPropagation::Private => "private",
            BindPropagation::RShared => "rshared",
            BindPropagation::RPrivate => "rprivate"
        };

        write!(f, "{}", string)
    }
}
