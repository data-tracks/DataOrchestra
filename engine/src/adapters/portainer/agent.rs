use crate::{object::Object, state::State, traits::Spawnable};

/// Representing a remote portainer agent
#[derive(Debug)]
pub struct Agent {
    pub object: Object,
}

impl Spawnable for Agent {
    fn state(&self) -> State {
        self.object.state()
    }

    fn build(&mut self) {
        self.object.build();
    }

    fn setup(&mut self) {
        self.object.setup();
    }

    fn deploy(&mut self) {
        self.object.deploy();
    }
}
