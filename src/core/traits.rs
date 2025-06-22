use super::object::Object;

pub trait Configurator {
    fn configure(&mut self, object: &mut Object);
}

pub trait Creator<T> {
    fn create(self) -> T;
}
