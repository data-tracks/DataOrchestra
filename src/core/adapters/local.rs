use std::process::{Command, Stdio};
use log::debug;
use crate::core::adapters::{Runner, RunnerError};

/// Local runner object. Executes commands on the local system
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

impl Runner for Local {
    fn exec(&self, command: String) -> Result<String, RunnerError> {
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

        if let Ok(result ) = output {
            if !result.status.success() {
                return Err(RunnerError::CommandRead(String::from_utf8(result.stderr).unwrap(), command));
            }
            Ok(String::from_utf8(result.stdout).unwrap())
        } 
        else if let Err(error) = output {
            Err(RunnerError::CommandExecute(error.to_string(), command))
        }
        else  {
            Err(RunnerError::CommandExecute("".to_string(), command))
        }
    }

    fn clone_box(&self) -> Box<dyn Runner + Send + Sync> {
        Box::new(self.clone()) 
    }
}
