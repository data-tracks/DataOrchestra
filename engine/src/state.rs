#[derive(Debug, Clone, Copy)]
pub enum State {
    NotRunning,
    Running,
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
