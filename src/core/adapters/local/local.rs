use std::process::{Command, Stdio};
use log::debug;
use crate::core::adapters::{Runner, Ssh};

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
    fn exec(&self, command: String) -> Result<String, String> {
        debug!("{}", format!("Running command [{}]", &command));
        let output;
        if cfg!(target_os = "windows") {
            output = Command::new("cmd")
                .arg("/C")
                .arg(command)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
        } else {
            output = Command::new("sh")
                .arg("-c")
                .arg(command)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
        }

        if let Ok(result ) = output {
            if !result.status.success() {
                return Err(String::from_utf8(result.stderr).unwrap());
            }
            return Ok(String::from_utf8(result.stdout).unwrap());
        } 
        else if let Err(error) = output {
            return Err(error.to_string());
        }
        else  {
            return Err("Unable to execute command".to_string());
        }
    }
}
