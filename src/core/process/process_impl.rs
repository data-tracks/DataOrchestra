use std::path::Path;
use std::thread::{self, JoinHandle};
use log::{info, debug, error};
use crate::core::adapters::docker::{ComposeGroupBuilder, DockerManager};
use crate::core::adapters::Executor;
use crate::core::process::process_types::ProcessTypeConfig;
use crate::core::utils::{start_ansible, start_script};
use crate::shared::traits::Start;
use crate::core::adapters::docker::Run;

use super::Process;

impl Start<()> for Process {
    /// Start initialisation process for store components
    ///
    /// # Note
    ///
    /// Calling `process.start()` moves the store into the thread
    fn start(mut self) -> JoinHandle<()> {
        debug!("Spawning process thread");
        
        thread::Builder::new().name("process".to_string()).spawn(move || {
            ////////////////////////////////////////////////////////////
            // Pre - processing
            ////////////////////////////////////////////////////////////

            let mut manager = DockerManager::new();
            if let Some(ref process) = self.process_type {
                debug!("Setting up {:?} enviroment", process);

                if self.config.is_none() {
                    info!("No config was provided. Setting up default config");
                    self.config = Some(self.process_type.as_ref().unwrap().new());
                }
                let config = self.config.as_ref().unwrap();

                let mut compose = ComposeGroupBuilder::new();
                config.setup_container(&mut compose);
                self.object.docker_group_builder = Some(compose);
            }

            // Take ownership of ContainerBuilder out of object to prevent partial move
            if let Some(group) = self.object.docker_group_builder.take() {
                info!("Setting up docker compose");
                let mut compose_group = group.build();
                let _result = compose_group.run();
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

            ////////////////////////////////////////////////////////////
            // Post - processing
            ////////////////////////////////////////////////////////////


            // Run ansible setup script on all containers
            for container in manager.as_vec() {
                if container.ssh.is_some() {
                    let result = start_ansible(container.get_ssh_port().unwrap()); 
                    if let Err(error) = result {
                        error!("{}", error);
                    }
                }
            }

            for (container, data) in manager.iter_combine_data(&self.object.data) {
                if let Some(ref ssh) = container.ssh {
                    info!("Uploading {} for {}", data.path, container.config.name.as_ref().unwrap());
                    let _ = ssh.upload_directory(&data.path, &data.destination);
                    if let Some(ref dependency) = data.dependency {
                        let dependency_path= Path::new(dependency);
                        let _ = ssh.upload_file(dependency_path, Path::new(&format!("/scripts/{}", dependency_path.display())));
                        let _ = ssh.exec(format!("sh /scripts/{}", dependency_path.file_name().unwrap().to_str().unwrap()));
                    }
                }
                else {
                    panic!("Ssh client unavailable");
                }
            }

            if let Some(ref config) = self.config {
                match config {
                    ProcessTypeConfig::Kafka(kafka) => {
                        if let Some(broker) = manager.containers.get("kafka-broker") {
                            kafka.create_topic(broker.id.as_ref().unwrap());
                        }
                        else {
                            panic!("Unable to find kafka-broker for kafka compose configuration");
                        }
                    },
                    _ => ()
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
