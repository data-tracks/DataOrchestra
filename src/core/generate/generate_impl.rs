use std::thread::{self, JoinHandle};
use log::{debug, info, error};
use crate::core::adapters::docker::{DockerManager, Run};
use crate::core::utils::{start_ansible, start_script};
use crate::shared::traits::Start;

use super::Generate;

impl Start<()> for Generate {
    /// Start initialisation process for store components
    ///
    /// # Note
    ///
    /// Calling `generate.start()` moves the store into the thread
    fn start(mut self) -> JoinHandle<()> {
        info!("Spawning generate thread");
        
        thread::Builder::new().name("generate".to_string()).spawn(move || {

            let mut manager = DockerManager::new();

            if let Some(group) = self.object.docker_group_builder.take() {
                debug!("Setting up compose");
                let compose_group = group.build();
                for container in compose_group.containers {
                    manager.add_container(container.config.name.as_ref().unwrap().clone(), container);
                } 
            }

            else if let Some(container) = self.object.docker_container_builder.take() {
                debug!("Setting up container");
                let mut container = container.build();
                let _ = container.run();
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


            info!("Finished");
        }).unwrap()
    }
}
