use serde::{Deserialize, Serialize};

/// The `Amount` type. Allows one to add a single value or vector of values in a json file
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Amount<T> {
    Single(T),
    Multiple(Vec<T>)
}

impl<T> Amount<T> {
    /// Check if amount has only one value
    pub fn has_one(&self) -> bool {
        matches!(self, Amount::Single(_))
    }

    /// Check if amount has multiple values
    pub fn has_multiple(&self) -> bool {
        matches!(self, Amount::Multiple(_))
    }

    /// Get the amount of items in the amount enum
    pub fn get_count(&self) -> usize {
        match self {
            Self::Single(_) => 1,
            Self::Multiple(values) => values.len()
        }
    }
}
