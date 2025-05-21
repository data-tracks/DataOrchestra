use std::fmt::Debug;
use crate::core::adapters::Ssh;

pub trait Runner: Debug where Self: 'static {
    fn exec(&self, command: String) -> Result<String, String>;
}

pub trait Uploader<T, S>: Debug {
    fn upload_file(&self, file: T, destination: S) -> Result<(), String>;
    fn upload_directory(&self, dir: T, destination: S) -> Result<(), String>;
}
