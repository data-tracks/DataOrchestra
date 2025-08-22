use std::fmt::Debug;
use std::path::Path;
use thiserror::Error;

/// Executor trait. Trait for system command execution
pub trait Executor: Debug where Self: 'static {
    /// Execute command
    fn exec(&self, command: String) -> Result<String, ExecutorError>;
    fn clone_box(&self) -> Box<dyn Executor + Send + Sync>;

    fn to_box_executor<'b>(&'b self) -> Box<dyn Executor + Send + Sync>
    where
        Self: Executor + Send + Clone,
        Self: 'b + Sync,
    {
        (Box::new(self.clone()) as Box<dyn Executor + Send + Sync>) as _
    }
}

#[derive(Debug, Error)]
pub enum ExecutorError {
    #[error("Unable to execute command (error {0}) (command {1})")] 
    CommandExecute(String, String),
    #[error("Unable to read command output (error {0}) (command {1})")]
    CommandRead(String, String),
    #[error("Unable to connect to session (error {0})")]
    SessionConnect(String),
    #[error("Unable to disconnect from session (error {0})")]
    SessionDisconnect(String)
}

impl From<ExecutorError> for String{
    fn from(value: ExecutorError) -> Self {
        value.to_string()
    }
}

#[derive(Debug, Error)]
pub enum UploaderError {
    #[error("No such directory exists ({0})")]
    NoSuchDirectory(String),
    #[error("No such file exists ({0})")]
    NoSuchFile(String),
    #[error("{0}")]
    UnableToUpload(String),
    #[error("Invalid File ({0})")]
    InvalidFile(String),
    #[error("Invalid Directory ({0})")]
    InvalidDirectory(String)
}

impl From<UploaderError> for String{
    fn from(value: UploaderError) -> Self {
        value.to_string()
    }
}

/// Uploader trait. Trait for data uploading
pub trait Uploader: Debug where Self: 'static {
    fn upload_file(&self, src: &Path, dst: &Path) -> Result<(), UploaderError>;
    fn upload_directory(&self, src: &Path, dst: &Path) -> Result<(), UploaderError>;

    fn to_box_uploader<'b>(&'b self) -> Box<dyn Uploader + Send + Sync>
    where
        Self: Uploader + Send + Clone,
        Self: 'b + Sync,
    {
        (Box::new(self.clone()) as Box<dyn Uploader + Send + Sync>) as _
    }
}
