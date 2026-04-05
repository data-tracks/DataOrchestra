use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum State {
    NotRunning,
    Running,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Running => "running",
            Self::NotRunning => "not running"
        };

        write!(f, "{}", s)
    }
}

impl Default for State {
    fn default() -> Self {
        Self::NotRunning
    }
}

impl State {
    pub fn is_running(&self) -> bool {
        matches!(self, State::Running)
    }

    pub fn is_not_running(&self) -> bool {
        matches!(self, State::NotRunning)
    }
}
