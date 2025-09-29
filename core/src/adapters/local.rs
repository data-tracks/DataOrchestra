use crate::adapters::{Executor, ExecutorError};
use log::debug;
use std::process::{Command, Stdio};

/// Local executor object. Executes commands on the local system
#[derive(Debug, Clone)]
pub struct Local {}

impl Local {
    pub fn new() -> Self {
        Local {}
    }

    pub fn new_box() -> Box<Self> {
        Box::new(Local::new())
    }
}

impl Executor for Local {
    fn exec(&self, command: String) -> Result<String, ExecutorError> {
        debug!("Running command [{}]", &command);
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .arg("/C")
                .arg(&command)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&command)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
        };

        if let Ok(result) = output {
            if !result.status.success() {
                return Err(ExecutorError::CommandRead(
                    String::from_utf8(result.stderr).unwrap(),
                    command,
                ));
            }
            Ok(String::from_utf8(result.stdout).unwrap())
        } else if let Err(error) = output {
            Err(ExecutorError::CommandExecute(error.to_string(), command))
        } else {
            Err(ExecutorError::CommandExecute("".to_string(), command))
        }
    }

    fn clone_box(&self) -> Box<dyn Executor + Send + Sync> {
        Box::new(self.clone())
    }
}
