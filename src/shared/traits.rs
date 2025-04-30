use std::thread::JoinHandle;

use super::Amount;

pub trait Start<T> {
    fn start(self) -> JoinHandle<T>;
}

/// Parsing external structure to internal structure
pub trait ToInternal<T> {
    fn to_internal(self) -> T;
}

impl<T> Amount<T> {
    pub fn to_internal<S>(self) -> Vec<S> where T: ToInternal<S> {
        let mut values_vec = Vec::<S>::new();
        match self {
            Amount::Multiple(values) => {
                for value in values {
                    values_vec.push(value.to_internal());
                }
            },
            Amount::Single(value) => {
                values_vec.push(value.to_internal());
            },
            Amount::None => {}
        } 

        values_vec
    }
}
