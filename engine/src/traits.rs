use super::state::State;

/// Represent an object which can configure its parent
pub trait Configurable<T> {
    fn configure(&mut self, parent: &mut T);
}

/// Represents an object which can be checked. This is normally implemented as a healthcheck
pub trait Checkable<T> {
    fn check(&self) -> Result<T, String>;
}

/// Represents an object which can be deployed
pub trait Spawnable {
    fn state(&self) -> State;
    fn build(&mut self);
    fn setup(&mut self);
    fn deploy(&mut self);
}
