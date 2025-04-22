use serde::{Deserialize, Serialize};

/// The `Amount` type. Allows a value to be nothing, one value or a collection on values
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Amount<T> {
    None,
    Single(T),
    Multiple(Vec<T>)
}

impl<T> Amount<T> {
    /// Check if amount has only one value
    pub fn has_one(&self) -> bool {
        matches!(self, Self::Single(_))
    }

    /// Check if amount has multiple values
    pub fn has_multiple(&self) -> bool {
        matches!(self, Self::Multiple(_))
    }

    /// Check if amount has a value or multiple
    pub fn has_something(&self) -> bool {
       self.has_one() || self.has_multiple() 
    }

    /// Check if amount has no value
    pub fn has_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Get the amount of items in the amount enum
    pub fn get_amount(&self) -> usize {
        match self {
            Self::None => 0,
            Self::Single(_) => 1,
            Self::Multiple(values) => values.len()
        }
    }
}

impl<T> Default for Amount<T> {
    fn default() -> Self {
        Amount::None
    }
}
