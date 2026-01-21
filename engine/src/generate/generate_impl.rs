use super::Generate;
use crate::state::State;
use crate::traits::Configurator;
use crate::traits::Spawner;

impl Spawner for Generate {
    fn state(&self) -> State {
        self.object.state()
    }

    fn build(&mut self) {
        /*
        if let Some(generate_type) = &self.generate_type {
            if self.config.is_none() {
                info!("No config was provided. Setting up default config");
                self.config = Some(.new());
            }
        }*/

        // Setup container based on specified config. Default setup if only process_type was provided,
        // otherwise custom
        if let Some(mut generate_config) = self.config.take() {
            generate_config.configure(self);
        }

        self.object.build();
    }

    fn setup(&mut self) {
        self.object.setup();
    }

    fn deploy(&mut self) {
        self.object.deploy();
    }
}
