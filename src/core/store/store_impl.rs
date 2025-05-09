use std::thread::{self, JoinHandle};
use log::{debug, info, error};
use crate::core::adapters::docker::{DockerManager, Run};
use crate::core::utils::{iter_combine_data, start_ansible, start_script, upload_data};
use crate::shared::traits::Spawner;

use super::Store;

impl Spawner<()> for Store {
    fn spawn(mut self) -> JoinHandle<()> {

        info!("Spawing");

        let thread = thread::Builder::new().name("store".to_string()).spawn(move || {
            let result = self.setup();
            if let Err(error) = result {
                panic!("{}", error);
            }

            let setup = result.unwrap();
            setup.deploy();
        }).unwrap();

        info!("Finished");

        thread
    }

    fn setup(mut self) -> Result<Self, String> where Self: Sized {
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
            info!("Setting up docker compose");
            let compose_group = group.build();
            for container in compose_group.containers {
                manager.add_container(container.config.name.as_ref().unwrap().clone(), container);
            }
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

        // Upload data to docker containers
        let result = upload_data(&manager,&self.object.data);
        if let Err(error) = result {
            panic!("Unable to upload data {}", error);
        }
        
        self.object.docker_manager = Some(manager);
        Ok(self)
    }

    fn deploy(mut self) -> Result<Self, String> where Self: Sized {
        if let Some(ref manager) = self.object.docker_manager {
            if manager.amount() > 1 {
                for (container, data) in iter_combine_data(manager, &self.object.data) {
                    if let Some(ref ssh) = container.ssh {
                        info!("Starting {} for {}", data.start, container.config.name.as_ref().unwrap());
                        let result = start_script(ssh, data);
                        if let Err(error) = result {
                            error!("Unable to start script | {}", error);
                        }
                    }
                    else {
                        panic!("Ssh client unavailable");
                    }
                }
            }
            else {
                if let Some(container) = manager.get_container(){
                    for data in self.object.data.iter() {
                        if let Some(ref ssh) = container.ssh {
                            info!("Starting {} for {}", data.start, container.config.name.as_ref().unwrap());
                            let result = start_script(ssh, data);
                            if let Err(error) = result {
                                error!("Unable to start script | {}", error);
                            }
                        } 
                        else {
                            panic!("Ssh client unavailable");
                        }
                    }
                }
            }
        }
        else if let Some(ref node) = self.object.node {
            
        }

        Ok(self)
    }
}
