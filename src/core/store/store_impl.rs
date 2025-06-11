use log::info;
use crate::core::adapters::ContainerBuilder;
use crate::shared::traits::Spawner;

use super::Store;

impl Spawner for Store {
    fn build(&mut self) {
        if let Some(db_type) = &self.db_type {
            if self.config.is_none() {
                info!("No config given for database type. Loading default config");
                self.config = Some(db_type.new());
            }
        }

        // Setup container based on specified config. Default setup if only db_type was provided,
        // otherwise custom
        if let Some(db_config) = self.config.as_mut() {
            let mut container = self.object
                .docker_container_builder
                .get_or_insert_with(ContainerBuilder::new);

            db_config.setup_container(&mut container);
            if self.schema.len() > 0 {
                db_config.mount_data(&self.schema, &mut container);
            }
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
