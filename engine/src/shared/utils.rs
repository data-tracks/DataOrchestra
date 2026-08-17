use std::{thread, time::Duration};
use crate::traits::Spawnable;

pub fn repeat_on_err<R, E, F>(f: F, amount: usize, sleep: Option<Duration>) -> Result<R, E>
    where F: Fn() -> Result<R, E>
{
    let mut result = f();
    for _ in 1..amount {
        if result.is_ok() {
            return result;
        }
        if let Some(time) = sleep {
            thread::sleep(time);
        }
        result = f();
    }

    result 
}  

pub fn repeat_on_err_mut<R, E, F>(mut f: F, amount: usize, sleep: Option<Duration>) -> Result<R, E>
    where F: FnMut() -> Result<R, E>
{
    let mut result = f();
    for _ in 1..amount {
        if result.is_ok() {
            return result;
        }
        if let Some(time) = sleep {
            thread::sleep(time);
        }
        result = f();
    }

    result 
}
