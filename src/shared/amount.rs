use std::{mem, vec::IntoIter};
use log::debug;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::Error;
use serde_json::Value;

/// The `Amount` type. Allows a value to be nothing, one value or a collection on values
/// Used to allow for variability of fields in the JSON schema
#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum Amount<T> {
    None,
    Single(T),
    Multiple(Vec<T>)
}

impl<T> Amount<T> {
    /// Check if amount has only one value
    #[inline]
    pub fn has_one(&self) -> bool {
        matches!(self, Self::Single(_))
    }

    /// Check if amount has multiple values
    #[inline]
    pub fn has_multiple(&self) -> bool {
        matches!(self, Self::Multiple(_))
    }

    /// Check if amount has a value or multiple
    #[inline]
    pub fn has_something(&self) -> bool {
       self.has_one() || self.has_multiple() 
    }

    /// Check if amount has no value
    #[inline]
    pub fn has_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Get the amount of items in the amount enum
    #[inline]
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
    #[inline]
    pub fn get_ref_single(&self) -> &T {
        match self {
            Self::Single(value) => value,
            _ => panic!("Get single on non single value"),
        }
    }

    /// Get mutable reference inside single enum
    ///
    /// # Panic
    ///
    /// When [`Amount`] is not of type `single`
    #[inline]
    pub fn get_ref_mut_single(&mut self) -> &mut T {
        match self {
            Self::Single(value) => value,
            _ => panic!("Get mut single on non single value"),
        }
    }

    /// Get reference inside multiple enum
    ///
    /// # Panic
    ///
    /// When [`Amount`] is not of type `multiple`
    #[inline]
    pub fn get_ref_multiple(&self) -> &Vec<T> {
        match self {
            Self::Multiple(values) => values,
            _ => panic!("Get ref multiple on non multiple value"),
        }
    }

    /// Get mutable reference inside multiple enum
    ///
    /// # Panic
    ///
    /// When [`Amount`] is not of type `multiple`
    #[inline]
    pub fn get_ref_mut_multiple(&mut self) -> &mut Vec<T> {
        match self {
            Self::Multiple(values) => values,
            _ => panic!("Get ref mut multiple on non multiple value"),
        }
    }

    /// Transform Amount enum value into vector
    #[inline]
    pub fn to_vec(self) -> Vec<T> {
        match self {
            Amount::None => Vec::new(),
            Amount::Single(value) => vec![value],
            Amount::Multiple(values) => values
        }
    } 
    
    /// Transform Amount enum value into vector of references
    #[inline]
    pub fn as_ref_vec(&self) -> Vec<&T> {
        match self {
            Amount::None => Vec::new(),
            Amount::Single(value) => vec![value],
            Amount::Multiple(values) => values.iter().collect::<Vec<&T>>()
        } 
    }

    /// Transform Amount enum value into vector of mutable references
    #[inline]
    pub fn as_mut_ref_vec(&mut self) -> Vec<&mut T> {
        match self {
            Amount::None => Vec::new(),
            Amount::Single(value) => vec![value],
            Amount::Multiple(values) => values.iter_mut().collect::<Vec<&mut T>>()
        } 
    }

    /// Take value out of the Amount enum
    #[inline]
    pub fn take(&mut self) -> Amount<T> {
        mem::replace(self, Amount::None)
    }

    /// Insert item into Amount enum
    #[inline]
    pub fn insert(&mut self, item: T) {
        // Take ownership of self by moving it out of memory. Insert value and then place result
        // back into self
        match mem::take(self) {
            Amount::None => {
                *self = Amount::Single(item);
            }
            Amount::Single(value) => {
                *self = Amount::Multiple(vec![value, item]);
            }
            Amount::Multiple(mut values) => {
                values.push(item);
                *self = Amount::Multiple(values);    
            }
        }
    }
}


impl<T> IntoIterator for Amount<T> {
    type Item = T;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
         self.to_vec().into_iter()
    }
}

impl<T> Default for Amount<T> {
    fn default() -> Self {
        Amount::None
    }
}

impl<'de, T> Deserialize<'de> for Amount<T> where T : Deserialize<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let value = Value::deserialize(deserializer)?;
        let first = value.to_string();

        if first.chars().next().is_some_and(|x| x == '[') {
            let result = Vec::deserialize(value.clone());
            if let Ok(compact) = result {
                return Ok(Amount::Multiple(compact));
            }
            else if let Err(error) = result {
                panic!("Error while deserializing Vec<T> [{}]", error);
            }
        }
        else {
            let result = T::deserialize(value.clone());
            if let Ok(full) = result {
                return Ok(Amount::Single(full));
            }
            else if let Err(error) = result {
                debug!("{}", value.clone());
                panic!("Error while deserializing T: [{}]", error);
            }
        }

        Err(Error::custom(
            "Could not deserialize into either a single store or a list of them",
        ))
    }
}

impl<T> PartialEq<Self> for Amount<T>
where
    T: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Amount::Single(one), Amount::Single(other)) => {
                one.eq(other)
            },
            (Amount::None, Amount::None) => {
                true
            },
            (Amount::Multiple(one), Amount::Multiple(other)) => {
                one.eq(other)
            },
            _ => false
        }
    }
}
