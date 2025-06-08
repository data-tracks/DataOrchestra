use std::fmt::Debug;

pub trait Runner: Debug where Self: 'static {
    fn exec(&self, command: String) -> Result<String, String>;
    fn clone_box(&self) -> Box<dyn Runner + Send + Sync>;

    fn to_box_runner<'b>(&'b self) -> Box<dyn Runner + Send + Sync>
    where
        Self: Runner + Send + Clone,
        Self: 'b + Sync,
    {
        (Box::new(self.clone()) as Box<dyn Runner + Send + Sync>) as _
    }
}

pub trait Uploader<T, S>: Debug where Self: 'static {
    fn upload_file(&self, file: T, destination: S) -> Result<(), String>;
    fn upload_directory(&self, dir: T, destination: S) -> Result<(), String>;

    fn to_box_uploader<'b>(&'b self) -> Box<dyn Uploader<T, S> + Send>
    where
        Self: Runner + Send + Clone,
        Self: 'b + Sync,
    {
        (Box::new(self.clone()) as Box<dyn Uploader<T, S> + Send>) as _
    }
}
