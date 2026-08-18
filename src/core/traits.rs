/// Represent an object which can configure its parent
pub trait Configurator<T> {
    fn configure(&mut self, parent: &mut T);
}

pub trait Creator<T, S> {
    fn create(self, o: &T) -> S;
}

/// Represents an object which can be checked. This is normally implemented as a healthcheck
pub trait Checkable<T> {
    fn check(&self) -> Result<T, String>;
}

/// Represents an object which can be deployed
pub trait Spawner {
    fn build(&mut self);
    fn setup(&mut self);
    fn deploy(&mut self);
}
