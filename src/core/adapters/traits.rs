use std::path::Path;

pub trait Executor {
    fn new() -> Self;
    fn connect(&mut self, host: &String, port: u16, username: &String, password: &String) -> Result<(), String>;
    fn upload_file<T: AsRef<Path>, S: AsRef<Path>>(&self, file: T, destination: S) -> Result<(), String>;
    fn upload_directory<T: AsRef<Path>, S: AsRef<Path>>(&self, dir: T, destination: S) -> Result<(), String>;
    fn exec<T: Into<String>>(&self, command: T) -> Result<String, String>;
}
