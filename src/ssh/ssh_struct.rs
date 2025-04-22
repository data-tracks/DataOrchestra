use core::fmt;

use ssh2::Session;

pub struct ssh {
    pub session: Session
}

impl fmt::Debug for ssh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ssh ignored")
    }
}
