use log::info;
use crate::core::adapters::docker::ComposeGroupBuilder;
use crate::core::adapters::{ContainerType, Local};
use crate::core::process::process_types::ProcessTypeConfig;
use crate::shared::traits::Spawner;

use super::Process;

impl Spawner for Process {
    fn build(&mut self) {
        if let Some(process_type) = &self.process_type {
            if self.config.is_none() {
                info!("No config was provided. Setting up default config");
                self.config = Some(process_type.new());
            }
        }
        
        // Setup container based on specified config. Default setup if only process_type was provided,
        // otherwise custom
        if let Some(process_config) = self.config.as_mut() {
            let mut compose = self.object.
                docker_group_builder.get_or_insert_with(ComposeGroupBuilder::new);

            if let Some(node) = self.object.node.as_ref() {
                match process_config {
                    ProcessTypeConfig::Kafka(ref mut kafka) => {
                        // Set enviroment variable KAFKA_HOST to node to make kafka broker
                        // accessible 
                        kafka.host = node.host.clone();

                        kafka.topics.push("orchestra-log".to_string());

                        compose
                            .name_mut("kafka-broker")
                            .name_mut("kafka-rest");
                    },
                    _ => ()
                }
            }

            

            process_config.setup_container(&mut compose);
        }

        self.object.build();
    } 

    fn setup(&mut self) {
        self.object.setup();
        
        // Start containers and move to manager
        if let Some(config) = self.config.as_mut() {
            match config {
                ProcessTypeConfig::Kafka(kafka) => {
                    match &self.object.docker_manager {
                        ContainerType::Compose(group) => {
                            if let Some(broker) = group.get_containers("kafka-broker") {
                                if let Some(ssh) = self.object.node.as_ref().and_then(|node| node.ssh.as_ref()) {
                                    kafka.create_topic(broker.id.as_ref().unwrap(), ssh);
                                }
                                else {
                                    kafka.create_topic(broker.id.as_ref().unwrap(), &Local::new());
                                }
                            }
                        }
                        _ => ()
                    }
                },
                _ => ()
            }
        }
    }

    fn deploy(&mut self) {
        self.object.deploy();
    }
}
