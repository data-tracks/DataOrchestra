use log::info;
use crate::core::adapters::{ContainerBuilder, Runner};
use crate::shared::traits::Spawner;

use super::Store;

impl Spawner for Store {
    fn build(&mut self) {
        info!("Building Store");

        // Create default config of specified database type `StoreType` if none was
        // specified
        if let Some(db_type) = &self.db_type {
            if self.config.is_none() {
                info!("No config given for database type. Loading default config");
                let mut container = ContainerBuilder::new();
                let config = db_type.new();
                config.setup_container(&mut container);
                if self.schema.len() > 0 {
                    config.mount_data(&self.schema, &mut container);
                }
                self.object.docker_container_builder = Some(container);
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
