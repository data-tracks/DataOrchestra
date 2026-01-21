
use core::fmt::Debug;

pub trait Run {
    type Output;
    type Error;

    fn run(&mut self) -> Result<Self::Output, Self::Error>;
} 

impl Debug for dyn Run<Output = (), Error = String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
