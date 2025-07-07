use core::fmt;
use log::trace;
use thiserror::Error;

use ssh2::Session;

/// The ssh object. Wrapper around the ssh2 [`Session`] object
pub struct Ssh {
    pub session: Session
}

impl Clone for Ssh {
    fn clone(&self) -> Self {
        trace!("CLONING SSH");
        Ssh { session: self.session.clone() }
    }
}

#[derive(Debug, Error)]
pub enum SshError {
    #[error("Unable to open tcp connection ({0})")]
    Tcp(String),
    #[error("Unable to handshake ({0})")]
    Handshake(String),
    #[error("Unable to authenticate session ({0})")]
    Authentication(String),
    #[error("Unknown error ({0})")]
    Unknown(String),
    #[error("Unable to execute command (error {0}) (command {1})")]
    CommandExecute(String, String),
    #[error("Unable to read output (error {0}) (command {1})")]
    CommandOutput(String, String),
    #[error("Unable to disconnect session ({0})")]
    UnableDisconnect(String),
    #[error("Unable to open ssh channel ({0})")]
    OpenChannel(String)
}



impl fmt::Debug for Ssh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ssh ignored")
    }
}
