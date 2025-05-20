use log::{error, info, warn};
use crate::core::adapters::docker::DockerManager;
use crate::core::adapters::{ContainerBuilder, ContainerType, Runner};
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

        let result = self.object.start_containers();
        if let Err(error) = result {
            // Unable to start containers is breaking.
            panic!("{}", error);
        }

        let result = self.object.start_ansible();
        if let Err(error) = result {
            // Unable to load ansible is not breaking. May fail if no ssh port is specified but that
            // may be a desired effect
            warn!("Unable to start ansible {}", error);
        }
        
        let result = self.object.upload_data();
        if let Err(error) = result {
            // Unable to upload data, scripts etc is breaking
            panic!("Unable to upload data {}", error);
        }

        info!("Finished setting up Store");
    }

    fn deploy(&mut self) {
        info!("Deploying Store");

        self.object.deploy(); 

        info!("Finished deploying Store");
    }
}
