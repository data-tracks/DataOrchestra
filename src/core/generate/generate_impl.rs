use std::path::Path;
use std::thread::{self, JoinHandle};
use log::{info, error};
use crate::core::adapters::docker::{DockerManager, Run};
use crate::core::adapters::Executor;
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
                info!("Setting up docker compose");
                let compose_group = group.build();
                for container in compose_group.containers {
                    manager.add_container(container.config.name.as_ref().unwrap().clone(), container);
                } 
            }

            else if let Some(container) = self.object.docker_container_builder.take() {
                info!("Setting up docker container");
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
                    info!("Uploading {} for {}", data.path, container.config.name.as_ref().unwrap());
                    let _ = ssh.upload_directory(&data.path, &data.destination);
                    if let Some(ref dependency) = data.dependency {
                        let dependency_path= Path::new(dependency);
                        let _ = ssh.exec("mkdir /scripts");
                        let _ = ssh.upload_file(dependency_path, Path::new(&format!("/scripts/{}", &dependency_path.file_name().unwrap().to_str().unwrap())));
                        let _ = ssh.exec(format!("sh /scripts/{}", dependency_path.file_name().unwrap().to_str().unwrap()));
                    }
                }
                else {
                    panic!("Ssh client unavailable");
                }
            }

            for (container, data) in manager.iter_combine_data(&self.object.data) {
                if let Some(ref ssh) = container.ssh {
                    info!("Starting {} for {}", data.path, container.config.name.as_ref().unwrap());
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
