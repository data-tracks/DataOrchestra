use log::{info, error};
use crate::core::adapters::docker::{DockerManager, Run};
use crate::core::utils::{iter_combine_data, start_ansible, start_script, upload_data};
use crate::shared::traits::Spawner;

use super::Generate;

impl Spawner for Generate {
    fn build(&mut self) {
        info!("Building Generate");

        if let Some(group) = self.object.docker_group_builder.take() {
            info!("Setting up docker compose");
            let compose_group = group.build();
            self.object.docker_group = Some(compose_group);
        }

        else if let Some(container) = self.object.docker_container_builder.take() {
            info!("Setting up docker container");
            let container = container.build();
            self.object.docker_container = Some(container);
        }

        info!("Finished building Generate");
    }

    fn setup(&mut self) {
        info!("Setting up Generate");

        self.object.docker_manager = Some(DockerManager::new());
            
        if let Some(ref mut manager) = self.object.docker_manager {
            if let Some(mut compose) = self.object.docker_group.take() {
                let _result = compose.run();
                for container in compose.containers {
                    manager.add_container(container.config.name.as_ref().unwrap().clone(), container);
                }
            }

            if let Some(mut container) = self.object.docker_container.take() {
                let _result = container.run();
                manager.add_container(container.config.name.as_ref().unwrap().clone(), container);
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

    }
    
    fn deploy(&mut self) {
        info!("Deploying Generate");
        
        if let Some(ref manager) = self.object.docker_manager {
            for (container, data) in iter_combine_data(manager, &self.object.data) {
                if let Some(ref ssh) = container.ssh {
                    info!("Starting {} for {}", data.start, container.config.name.as_ref().unwrap());
                    let result = start_script(ssh, data);
                    if let Err(error) = result {
                        error!("Unable to start script {} | {}", data.start, error);
                    }
                }
                else {
                    panic!("Ssh client unavailable");
                }
            }
        }

        info!("Finished deploying Generate");
    }
}
