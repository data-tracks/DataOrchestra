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

    /// Get reference value inside single enum
    ///
    /// # Panic
    ///
    /// When [`Amount`] is not of type `single`
    pub fn get_ref_single(&self) -> &T {
        match self {
            Self::Single(ref value) => value,
            _ => panic!("Get single on non single value"),
        }
    }

    /// Get mutable reference inside single enum
    ///
    /// # Panic
    ///
    /// When [`Amount`] is not of type `single`
    pub fn get_ref_mut_single(&mut self) -> &mut T {
        match self {
            Self::Single(ref mut value) => value,
            _ => panic!("Get mut single on non single value"),
        }
    }

    /// Get reference inside multiple enum
    ///
    /// # Panic
    ///
    /// When [`Amount`] is not of type `multiple`
    pub fn get_ref_multiple(&self) -> &Vec<T> {
        match self {
            Self::Multiple(ref values) => values,
            _ => panic!("Get ref multiple on non multiple value"),
        }
    }

    /// Get mutable reference inside multiple enum
    ///
    /// # Panic
    ///
    /// When [`Amount`] is not of type `multiple`
    pub fn get_ref_mut_multiple(&mut self) -> &mut Vec<T> {
        match self {
            Self::Multiple(ref mut values) => values,
            _ => panic!("Get ref mut multiple on non multiple value"),
        }
    }

    /// Transform Amount enum value into vector
    pub fn to_vec(self) -> Vec<T> {
        match self {
            Amount::None => Vec::new(),
            Amount::Single(value) => vec![value],
            Amount::Multiple(values) => values
        }
    } 
}

impl<T> Default for Amount<T> {
    fn default() -> Self {
        Amount::None
    }
}

