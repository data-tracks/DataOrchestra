use crate::interface::general::General;

pub trait Configurator<T> {
    fn configure(&mut self, parent: &mut T);
}

pub trait Creator<T> {
    fn create(self, general: &General) -> T;
}

pub trait Checkable<T> {
    fn check(&self) -> Result<T, String>;
}
