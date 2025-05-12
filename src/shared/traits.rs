use std::thread::{JoinHandle, ScopedJoinHandle};

use super::Amount;

pub trait Spawner<T> {
    fn build(&mut self);
    fn setup(self) -> JoinHandle<Self> where Self: Sized;
    fn deploy(self) -> JoinHandle<Self> where Self: Sized;
}

/// Parsing external structure to internal structure
pub trait ToInternal<T> {
    fn to_internal(self) -> T;
}

/// Parsing external structure to internal structure, when one object is capable of creating copies
/// of itself
pub trait ToInternalVec<T> {
    fn to_internal(self) -> Vec<T>;
}


// ToInternal implementation for the Amount enum to get Vec<S -> T>
impl<T, S> ToInternal<Vec<T>> for Amount<S> 
where 
    S: ToInternal<T>
{
    fn to_internal(self) -> Vec<T> {
        let mut values_vec = Vec::<T>::new();
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



// ToInternal implementation for the Amount enum to get Vec<S -> Vec<T>>. This is specifically for
// objects which implement the ToInternalVec, due to the `amount
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


