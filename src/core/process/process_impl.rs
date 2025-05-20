use log::{info, debug, error};
use crate::core::adapters::docker::{ComposeGroupBuilder, Run};
use crate::core::adapters::ContainerType;
use crate::core::process::process_types::ProcessTypeConfig;
use crate::shared::traits::Spawner;

use super::Process;

impl Spawner for Process {
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

        self.object.build();

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

            if let Some(ref config) = self.config {
                match config {
                    ProcessTypeConfig::Kafka(kafka) => {
                        if let Some(broker) = manager.containers.get("kafka-broker") {
                            match broker {
                                ContainerType::Container(broker) => {
                                    if let Some(ssh) = &broker.ssh {
                                        kafka.create_topic(broker.id.as_ref().unwrap(), &ssh.to_box_runner());
                                    }
                                }
                                _ => ()
                            }
                        }
                        else {
                            panic!("Unable to find kafka-broker for kafka compose configuration");
                        }
                    },
                    _ => ()
                }
            }
        }

        let result = self.object.start_ansible();
        if let Err(error) = result {
            error!("Unable to start ansible {}", error);
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

        self.object.deploy();

        info!("Finished deploying Process");
    }
}
