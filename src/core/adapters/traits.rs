use std::fmt::Debug;
use thiserror::Error;

/// Runner trait. Trait for system execution objects
pub trait Runner: Debug where Self: 'static {
    /// Execute command
    fn exec(&self, command: String) -> Result<String, RunnerError>;
    fn clone_box(&self) -> Box<dyn Runner + Send + Sync>;

    fn to_box_runner<'b>(&'b self) -> Box<dyn Runner + Send + Sync>
    where
        Self: Runner + Send + Clone,
        Self: 'b + Sync,
    {
        (Box::new(self.clone()) as Box<dyn Runner + Send + Sync>) as _
    }
}

#[derive(Debug, Error)]
pub enum RunnerError {
    #[error("Unable to execute command (error {0}) (command {1})")] 
    CommandExecute(String, String),
    #[error("Unable to read command output (error {0}) (command {1})")]
    CommandRead(String, String),
    #[error("Unable to connect to session (error {0})")]
    SessionConnect(String),
    #[error("Unablt to disconnect from session (error {0})")]
    SessionDisconnect(String)
}

impl From<RunnerError> for String{
    fn from(value: RunnerError) -> Self {
        value.to_string()
    }
}

/// Uploader trait. Trait for file uploading
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
