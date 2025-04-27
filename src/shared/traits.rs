use std::thread::JoinHandle;

pub trait Start<T> {
    fn start(self) -> JoinHandle<T>;
}

pub trait ToInternal<T> {
    fn to_internal(self) -> T;
}
