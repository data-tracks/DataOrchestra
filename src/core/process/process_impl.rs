use log::{info, debug, error};
use crate::core::adapters::docker::{ComposeGroupBuilder, DockerManager, Run};
use crate::core::adapters::{ContainerType, Runner};
use crate::shared::traits::Spawner;

use super::Process;

impl Spawner for Process {
    fn build(&mut self) {
        info!("Building Process");
        
        let mut manager = DockerManager::new();
        dbg!(&self);

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
            let compose = group.build();
            manager.add("group", ContainerType::Compose(compose));
        }
        else if let Some(container) = self.object.docker_container_builder.take() {
            info!("Setting up docker container");
            let mut container = container.build();

            if let Some(ref node) = self.object.node {
                if let Some(ref ssh) = node.ssh {
                    let runner = Box::new(ssh.clone()) as Box<dyn Runner + Send>;
                    container.runner = runner;
                }
            }
            manager.add(container.config.name.clone().unwrap(), ContainerType::Container(container));
        }

        self.object.docker_manager = Some(manager);
        dbg!(&self.object.docker_manager);

        info!("Finished building Process");
    } 

    fn setup(&mut self) {
        info!("Setting up Process");

        // Start containers and move to manager
        if let Some(ref mut manager) = self.object.docker_manager {
            for (name, item) in manager.containers.iter_mut() {
                debug!("Running docker {}", name);
                let result = item.run();
                if let Err(error) = result {
                    panic!("{}", error);
                }
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

        let result = self.object.start_ansible();
        if let Err(error) = result {
            panic!("Unable to start ansible {}", error);
        }

        // Upload data to docker containers
        let result = self.object.upload_data();
        if let Err(error) = result {
            panic!("Unable to upload data {}", error);
        }

        info!("Finished setting up Process");
    }

    fn deploy(&mut self) {
        info!("Deploying Process");

        let result = self.object.start_script();
        if let Err(error) = result {
            error!("{}", error);
        }

        info!("Finished deploying Process");
    }
}
