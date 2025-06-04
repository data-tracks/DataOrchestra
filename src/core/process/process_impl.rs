use log::info;
use crate::core::adapters::docker::ComposeGroupBuilder;
use crate::core::adapters::{ContainerType, Local, Runner};
use crate::core::process::process_types::ProcessTypeConfig;
use crate::shared::traits::Spawner;

use super::Process;

impl Spawner for Process {
    fn build(&mut self) {
        info!("Building Process");

        if let Some(process_type) = &self.process_type {
            if self.config.is_none() {
                info!("No config was provided. Setting up default config");
                self.config = Some(process_type.new());
            }
        }
        
        // Setup container based on specified config. Default setup if only process_type was provided,
        // otherwise custom
        if let Some(process_config) = self.config.as_mut() {
            if let Some(node) = self.object.node.as_ref() {
                match process_config {
                    ProcessTypeConfig::Kafka(ref mut kafka) => {
                        // Set enviroment variable KAFKA_HOST to node to make kafka broker
                        // accessible 
                        kafka.host = node.host.clone();
                    },
                    _ => ()
                }
            }

            let mut compose = self.object.
                docker_group_builder.get_or_insert_with(ComposeGroupBuilder::new);

            process_config.setup_container(&mut compose);
        }

        self.object.build();

        info!("Finished building Process");
    } 

    fn setup(&mut self) {
        info!("Setting up Process");

        self.object.setup();
        
        // Start containers and move to manager
        if let Some(config) = self.config.as_mut() {
            match config {
                ProcessTypeConfig::Kafka(kafka) => {
                    match &self.object.docker_manager {
                        ContainerType::Compose(group) => {
                            if let Some(broker) = group.get_container("kafka-broker") {
                                if let Some(ssh) = self.object.node.as_ref().and_then(|node| node.ssh.as_ref()) {
                                    kafka.create_topic(broker.id.as_ref().unwrap(), &ssh.to_box_runner());
                                }
                                else {
                                    kafka.create_topic(broker.id.as_ref().unwrap(), &Local::new().to_box_runner());
                                }
                            }
                        }
                        _ => ()
                    }
                },
                _ => ()
            }
        }

        info!("Finished setting up Process");
    }

    fn deploy(&mut self) {
        info!("Deploying Process");

        self.object.deploy();

        info!("Finished deploying Process");
    }
}
