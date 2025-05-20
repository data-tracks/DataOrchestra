use log::{debug, error, info};
use crate::core::adapters::docker::{DockerManager, Run};
use crate::core::adapters::{ContainerType, Runner};
use crate::shared::traits::Spawner;

use super::Store;

impl Spawner for Store {
    fn build(&mut self) {
        info!("Building Store");

        let mut manager = DockerManager::new();

        // Create default config of specified database type `StoreType` if none was
        // specified
        if self.config.is_none() && self.db_type.is_some(){
            info!("No config given for database type. Loading default config");
            self.config = Some(self.db_type.as_ref().unwrap().new());
        }

        // Take ownership of ComposeGroupBuilder out of object to prevent partial move 
        if let Some(group) = self.object.docker_group_builder.take() {
            info!("Setting up docker compose");
            let compose = group.build();  
            manager.add("group",ContainerType::Compose(compose));
        }  
        // Take ownership of ContainerBuilder out of object to prevent partial move
        else if let Some(mut container) = self.object.docker_container_builder.take() {
            info!("Setting up docker container");
            if let Some(ref database) = self.db_type {
                let mut db_config = &database.new();
                if let Some(ref mut config) = self.config {
                    db_config = config;
                }
                db_config.setup_container(&mut container); 

                if self.schema.len() > 0 {
                    db_config.mount_data(&self.schema, &mut container);
                }
            }

            let mut container = container.build();
            if let Some(ref node) = self.object.node {
                if let Some(ref ssh) = node.ssh {
                    let runner = Box::new(ssh.clone()) as Box<dyn Runner + Send>;
                    container.runner = runner;
                } 
            }
            manager.add(container.config.name.clone().unwrap(), ContainerType::Container(container));
        }

        self.object.docker_manager = Some(manager);
        dbg!(&self.object.docker_manager);

        info!("Finished building Store");
    }

    fn setup(&mut self) {
        info!("Setting up Store");

        // Start containers and move to manager
        if let Some(ref mut manager) = self.object.docker_manager {
            for (name, item) in manager.containers.iter_mut() {
                debug!("Running docker {}", name);
                let result = item.run();
                if let Err(error) = result {
                    panic!("{}", error);
                }
            }
        }

        let result = self.object.start_ansible();
        if let Err(error) = result {
            panic!("Unable to start ansible {}", error);
        }

        // Upload data to docker containers
        let result = self.object.upload_data();
        if let Err(error) = result {
            panic!("Unable to upload data {}", error);
        }

        info!("Finished setting up Store");
    }

    fn deploy(&mut self) {
        info!("Deploying Store");

        let result = self.object.start_script();
        if let Err(error) = result {
            error!("{}", error);
        }

        info!("Finished deploying Store");
    }
}
