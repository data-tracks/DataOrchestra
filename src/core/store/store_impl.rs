use log::info;
use crate::core::adapters::ContainerBuilder;
use crate::shared::traits::Spawner;

use super::Store;

impl Spawner for Store {
    fn build(&mut self) {
        info!("Building Store");

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

        info!("Finished building Store");
    }

    fn setup(&mut self) {
        info!("Setting up Store");

        self.object.setup();
        
        info!("Finished setting up Store");
    }

    fn deploy(&mut self) {
        info!("Deploying Store");

        self.object.deploy(); 

        info!("Finished deploying Store");
    }
}
