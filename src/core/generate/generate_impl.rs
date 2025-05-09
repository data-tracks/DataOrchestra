use std::path::Path;
use std::thread::{self, JoinHandle};
use log::{info, error};
use crate::core::adapters::docker::{DockerManager, Run};
use crate::core::adapters::Executor;
use crate::core::utils::{iter_combine_data, start_ansible, start_script};
use crate::shared::traits::Spawner;

use super::Generate;

impl Spawner<()> for Generate {
    fn spawn(mut self) -> JoinHandle<()> {

        info!("Spawing");

        let thread = thread::Builder::new().name("generate".to_string()).spawn(move || {
            let result = self.setup();
            if let Err(error) = result {
                panic!("{}", error);
            }

            let setup = result.unwrap();
            let _result = setup.deploy();

        }).unwrap();

        info!("Finished");

        thread
    }

    fn setup(mut self) -> Result<Self, String> where Self: Sized {
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
        for (container, data) in iter_combine_data(&manager, &self.object.data) {
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

        Ok(self) 
    }

    /// Start initialisation process for store components
    ///
    /// # Note
    ///
    /// Calling `generate.start()` moves the store into the thread
    fn deploy(mut self) -> Result<Self, String> where Self: Sized {
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


        info!("Finished");

        Ok(self)
    }
}
