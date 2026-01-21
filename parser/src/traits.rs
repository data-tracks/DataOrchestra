/// Represents an object which can create an object
pub trait Creatable<T, S> {
    fn create(self, o: &T) -> S;
}


use super::amount::Amount;

/// Parsing external structure to internal structure
pub trait ToInternal<T> {
    fn to_internal(self) -> T;
}

/// Parsing external structure to internal structure, when one object is capable of creating copies
/// of itself
pub trait ToInternalVec<T> {
    fn to_internal(self) -> Vec<T>;
}

impl<T, S> ToInternal<Option<T>> for Option<S> where S: ToInternal<T> {
    fn to_internal(self) -> Option<T> {
        self.map(|item| item.to_internal())
    }
}

/// ToInternal implementation for the Amount enum to get Vec<S -> T>
impl<T, S> ToInternal<Vec<T>> for Amount<S>
where
    S: ToInternal<T>
{
    fn to_internal(self) -> Vec<T> {
        self.into_iter()
            .map(|item| item.to_internal())
            .collect()
    }
}

/// ToInternal implementation for the Amount enum to get Vec<S -> Vec<T>>. This is specifically for
/// objects which implement the ToInternalVec
impl<T, S> ToInternalVec<T> for Amount<S>
where
    S: ToInternalVec<T>,
{
    fn to_internal(self) -> Vec<T> {
        let mut values_vec = Vec::<T>::new();
        match self {
            Amount::Multiple(values) => {
                for value in values {
                    values_vec.extend(value.to_internal());
                }
            },
            Amount::Single(value) => {
                values_vec.extend(value.to_internal());
            },
            Amount::None => {}
        }

        values_vec
    }
}

