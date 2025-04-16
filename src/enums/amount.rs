use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Amount<T> {
    Single(T),
    Multiple(Vec<T>)
}

impl<T> Amount<T> {
    pub fn amount(&self) -> usize {
        match self {
            Self::Multiple(ref values) => values.len() ,
            _ => 1
        }
    }
}
