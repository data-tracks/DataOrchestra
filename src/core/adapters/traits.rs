use std::fmt::Debug;

pub trait Runner: Debug {
    fn exec(&self, command: String) -> Result<String, String>;
}

pub trait Uploader<T, S>: Debug {
    fn upload_file(&self, file: T, destination: S) -> Result<(), String>;
    fn upload_directory(&self, dir: T, destination: S) -> Result<(), String>;
}
