use std::thread::{self, JoinHandle};
use log::{debug, info, error};
use crate::core::adapters::docker::{DockerManager, Run};
use crate::core::utils::{start_ansible, start_script};
use crate::shared::traits::Start;

use super::Store;

impl Start<()> for Store {
    /// Start initialisation process for store components
    ///
    /// # Note
    ///
    /// Calling `store.start()` moves the store into the thread
    fn start(mut self) -> JoinHandle<()> {
        debug!("Spawning storing thread");
        thread::Builder::new().name("store".to_string()).spawn(move || {
            // Create default config of specified database type ([`StoreType`]) if none was
            // specified
            if self.config.is_none() && self.db_type.is_some(){
                info!("No config given for database type. Loading default config");
                self.config = Some(self.db_type.as_ref().unwrap().new());
            }

            // Create and run containers
            let mut manager = DockerManager::new();

            // Take ownership of ComposeGroupBuilder out of object to prevent partial move 
            if let Some(group) = self.object.docker_group_builder.take() {
                debug!("Setting up compose");
                let compose_group = group.build();
                for container in compose_group.containers {
                    manager.add_container(container.config.name.as_ref().unwrap().clone(), container);
                }
            }  

            // Take ownership of ContainerBuilder out of object to prevent partial move
            else if let Some(mut container) = self.object.docker_container_builder.take() {
                debug!("Setting up container");
                if let Some(ref database) = self.db_type {
                    let mut db_config = &database.new();
                    if let Some(ref mut config) = self.config {
                        db_config = config;
                    }
                    db_config.setup_container(&mut container); 

                    if self.schema.len() > 0 {
                        //TODO: Maybe remove clone
                        db_config.mount_data(&self.schema, &mut container);
                    }
                }

                let mut container = container.build();
                let _ = container.run();
                manager.add_container(container.config.name.as_ref().unwrap().clone(), container);
            }

            // Run ansible setup script on all containers
            for container in manager.as_vec() {
                if container.ssh.is_some() {
                    let result = start_ansible(container.get_ssh_port().unwrap()); 
                    if let Err(error) = result {
                        error!("{}", error);
                    }
                }
            }

            // Upload data directory
            for (container, data) in manager.iter_combine_data(&self.object.data) {
                if let Some(ref ssh) = container.ssh {
                    let _ = ssh.upload_directory(&data.path, &data.destination);
                }
                else {
                    panic!("Ssh client unavailable");
                }
            }

            for (container, data) in manager.iter_combine_data(&self.object.data) {
                if let Some(ref ssh) = container.ssh {
                    start_script(ssh, data);
                }
                else {
                    panic!("Ssh client unavailable");
                }
            }

            // Start script
            info!("Finished");
        }).unwrap()
    }
}
