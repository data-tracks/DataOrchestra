use log::{info, debug, error};
use crate::core::adapters::docker::{ComposeGroupBuilder, DockerManager, Run};
use crate::core::adapters::{ContainerType, Runner};
use crate::core::utils::{iter_combine_data, start_ansible, start_script, upload_data};
use crate::shared::traits::Spawner;

use super::Process;

impl Spawner for Process {
    fn build(&mut self) {
        info!("Building Process");
        
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
        }

        // Take ownership of ContainerBuilder out of object to prevent partial move
        if let Some(group) = self.object.docker_group_builder.take() {
            info!("Setting up docker compose");
            let compose = group.build();
            manager.add("", ContainerType::Compose(compose));
        }
        else if let Some(container) = self.object.docker_container_builder.take() {
            info!("Setting up docker container");
            let mut container = container.build();

            if let Some(ref node) = self.object.node {
                let ssh: Option<Box<dyn Runner + Send>> = match &node.ssh {
                    Some(item) => Some(Box::new(item.clone())),
                    None => panic!()
                };

                container.runner = ssh;
            }
            manager.add(container.config.name.clone().unwrap(), ContainerType::Container(container));
        }

        info!("Finished building Process");
    } 

    fn setup(&mut self) {
        info!("Setting up Process");

        self.object.docker_manager = Some(DockerManager::new());

        // Start containers and move to manager
        if let Some(ref mut manager) = self.object.docker_manager {
            for (_, item) in manager.containers.iter_mut() {
                let result = item.run();
                if let Err(error) = result {
                    panic!("{}", error);
                }
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

            /*
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
            */
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
