use crate::state::State;
use crate::traits::Configurator;
use crate::traits::Spawner;
use log::info;

use super::Store;

impl Spawner for Store {
    fn state(&self) -> State {
        self.object.state()
    }

    fn build(&mut self) {
        if let Some(db_type) = &self.db_type
            && self.config.is_none()
        {
            info!("No config given for database type. Loading default config");
            self.config = Some(db_type.get_default());
        }

        // Setup container based on specified config. Default setup if only db_type was provided,
        // otherwise custom
        if let Some(mut db_config) = self.config.take() {
            let _ = self.object.docker_container_builder.get_or_insert_default();

            db_config.configure(self);
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
