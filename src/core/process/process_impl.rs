use log::{info, debug, error};
use crate::core::adapters::docker::{ComposeGroupBuilder, DockerManager, Run};
use crate::core::process::process_types::ProcessTypeConfig;
use crate::core::utils::{iter_combine_data, start_ansible, start_script, upload_data};
use crate::shared::traits::Spawner;

use super::Process;

impl Spawner<()> for Process {
    fn build(&mut self) {
        info!("Building Process");
       
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
            let compose_group = group.build();
            self.object.docker_group = Some(compose_group); 
        }
        else if let Some(container) = self.object.docker_container_builder.take() {
            info!("Setting up docker container");
            let container = container.build();
            self.object.docker_container = Some(container); 
        }

        info!("Finished building Process");
    } 

    fn setup(&mut self) {
        info!("Setting up Process");

        self.object.docker_manager = Some(DockerManager::new());

        // Start containers and move to manager
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
        }

        info!("Finished setting up Process");
    }

    fn deploy(&mut self) {
        info!("Deploying Process");

        if let Some(ref manager) = self.object.docker_manager {
            for (container, data) in iter_combine_data(manager, &self.object.data) {
                if let Some(ref ssh) = container.ssh {
                    info!("Starting {} for {}", data.path, container.config.name.as_ref().unwrap());
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

        info!("Finished deploying Process");
    }
}
