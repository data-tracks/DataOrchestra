use std::thread::{self, JoinHandle};
use log::{info, error};
use crate::core::adapters::docker::{DockerManager, Run};
use crate::core::adapters::Executor;
use crate::core::utils::{iter_combine_data, start_ansible, start_script, upload_data};
use crate::shared::traits::Spawner;

use super::Generate;

impl Spawner<()> for Generate {
    fn build(mut self) -> JoinHandle<Self> where Self: Sized {
        thread::Builder::new().name("generate".to_string()).spawn(move || { 
            info!("Building Generate");

            let mut manager = DockerManager::new();

            if let Some(group) = self.object.docker_group_builder.take() {
                info!("Setting up docker compose");
                let compose_group = group.build();
                for container in compose_group.containers {
                    manager.add_container(container.config.name.as_ref().unwrap().clone(), container);
                } 
            }

            else if let Some(container) = self.object.docker_container_builder.take() {
                info!("Setting up docker container");
                let container = container.build();
                manager.add_container(container.config.name.as_ref().unwrap().clone(), container);
            }

            self.object.docker_manager = Some(manager);

            info!("Finished building Generate");

            self
        }).unwrap()
    }

    fn setup(mut self) -> JoinHandle<Self> where Self: Sized {
        thread::Builder::new().name("generate".to_string()).spawn(move || {
            info!("Setting up Generate");
                
            if let Some(ref mut manager) = self.object.docker_manager {
                for (_, container) in manager.containers.iter_mut() {
                    container.run();
                }

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
            }

            info!("Finished setting up Generate");

            self
        }).unwrap()
    }

    /// Start initialisation process for store components
    ///
    /// # Note
    ///
    /// Calling `generate.start()` moves the store into the thread
    fn deploy(mut self) -> JoinHandle<Self> where Self: Sized {
        thread::Builder::new().name("generate".to_string()).spawn(move || {
            info!("Deploying Generate");
            
            if let Some(ref manager) = self.object.docker_manager {
                for (container, data) in iter_combine_data(manager, &self.object.data) {
                    if let Some(ref ssh) = container.ssh {
                        info!("Starting {} for {}", data.path, container.config.name.as_ref().unwrap());
                        start_script(ssh, data);
                    }
                    else {
                        panic!("Ssh client unavailable");
                    }
                }
            }

            info!("Finished deploying Generate");

            self
        }).unwrap()
    }
}
