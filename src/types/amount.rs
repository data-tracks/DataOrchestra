use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Amount<T> {
    Single(T),
    Multiple(Vec<T>)
}

impl<T> Amount<T> {
    pub fn has_one(&self) -> bool {
        matches!(self, Amount::Single(_))
    }

    pub fn has_multiple(&self) -> bool {
        matches!(self, Amount::Multiple(_))
    }
}
