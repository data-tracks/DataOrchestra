use std::thread::{self, JoinHandle};
use log::{info, debug, error};
use crate::core::adapters::docker::{ComposeGroupBuilder, DockerManager, Run};
use crate::core::process::process_types::ProcessTypeConfig;
use crate::core::utils::{iter_combine_data, start_ansible, start_script, upload_data};
use crate::shared::traits::Spawner;

use super::Process;

impl Spawner<()> for Process {
    fn spawn(mut self) -> JoinHandle<()> {

        info!("Spawing");

        let thread = thread::Builder::new().name("process".to_string()).spawn(move || {
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

        let mut manager = DockerManager::new();

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
        
        self.object.docker_manager = Some(manager);
        Ok(self)
    }

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

        Ok(self)
    }
}
