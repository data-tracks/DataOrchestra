use core::fmt;

use ssh2::Session;

pub struct Ssh {
    pub session: Session
}

impl fmt::Debug for Ssh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ssh ignored")
    }
}
