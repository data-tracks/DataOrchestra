use log::info;
use crate::core::adapters::{ContainerType, Local};
use crate::core::process::process_types::ProcessTypeConfig;
use crate::core::traits::Configurator;
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
        if let Some(mut process_config) = self.config.take(){
            let _ = self.object.docker_group_builder
                .get_or_insert_default();

            process_config.configure(self);
        }

        self.object.build();
    } 

    fn setup(&mut self) {
        self.object.setup();
       
        // Create kafka topic
        if let Some(config) = self.config.as_mut() {
            if let ProcessTypeConfig::Kafka(kafka) = config {
                if let ContainerType::Compose(group) = &self.object.docker_manager {
                    if let Some(broker) = group.get_containers("kafka-broker") {
                        if let Some(ssh) = self.object.node.as_ref().and_then(|node| node.ssh.as_ref()) {
                            kafka.create_topic(broker.id.as_ref().unwrap(), ssh);
                        }
                        else {
                            kafka.create_topic(broker.id.as_ref().unwrap(), &Local::new());
                        }
                    }
                }
            }
        }
    }

    fn deploy(&mut self) {
        self.object.deploy();
    }
}
