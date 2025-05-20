use core::panic;
use std::net::{IpAddr, Ipv4Addr};

use log::{debug, error, info};
use crate::core::adapters::docker::{DockerManager, Run};
use crate::core::adapters::{ContainerType, Runner, Uploader};
use crate::shared::traits::Spawner;

use super::Generate;

impl Spawner for Generate {
    fn build(&mut self) {
        info!("Building Generate");

        let mut manager = DockerManager::new();

        if let Some(ref mut node) = self.object.node {
            let result = node.load_ssh();
            if let Err(error) = result {
                panic!("Unable to setup ssh for {} {}", node.host, error);
            }
        }

        if let Some(group) = self.object.docker_group_builder.take() {
            info!("Setting up docker compose");
            let compose = group.build();
            //TODO: Key name for compose
            manager.add("", ContainerType::Compose(compose));
        }

        else if let Some(container) = self.object.docker_container_builder.take() {
            info!("Setting up docker container");
            let mut container = container.build();
            // TODO: Add local runner
            if let Some(ref node) = self.object.node {
                if let Some(ref ssh) = node.ssh {
                    let runner = Box::new(ssh.clone()) as Box<dyn Runner + Send>;
                    container.runner = runner;
                }
            }
            manager.add(container.config.name.clone().unwrap(), ContainerType::Container(container));
        }

        self.object.docker_manager = Some(manager);

        info!("Finished building Generate");
    }

    fn setup(&mut self) {
        info!("Setting up Generate");
        if let Some(ref node) = self.object.node {
            if let Some(ref ssh) = node.ssh {
                let result = ssh.upload_directory("scripts", "/home/ubuntu/scripts");
                if let Err(error) = result {
                    error!("{}", error);
                }
            }
        }

        if let Some(ref mut manager) = self.object.docker_manager {
            for (name, item) in manager.containers.iter_mut() {
                debug!("Running docker {}", name);
                let result = item.run();
                if let Err(error) = result {
                    panic!("{}", error);
                }
                match item { 
                    ContainerType::Compose(compose) => {
                        for container in compose.containers.iter_mut() {
                            if let Some(ref node) = self.object.node {
                                let result = container.load_ssh(node.host);
                                if let Err(error) = result {
                                    error!("Unable to load ssh connection for {} {}", node.host, error);
                                }        
                            }
                            else {
                                let result = container.load_ssh(IpAddr::V4(Ipv4Addr::LOCALHOST));
                                if let Err(error) = result {
                                    error!("Unable to load ssh connection for localhost {}", error);
                                }  
                            }
                        }
                    }
                    ContainerType::Container(container ) => {
                        if let Some(ref node) = self.object.node {
                            let result = container.load_ssh(node.host);
                            if let Err(error) = result {
                                error!("Unable to load ssh connection for {} {}", node.host, error);
                            }    
                        }
                        else {
                            let result = container.load_ssh(IpAddr::V4(Ipv4Addr::LOCALHOST));
                            if let Err(error) = result {
                                error!("Unable to load ssh connection for localhost {}", error);
                            } 
                        }
                    }
                }
            }
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

        info!("Finished setting up Generate");

    }
    
    fn deploy(&mut self) {
        info!("Deploying Generate");
        
        let result = self.object.start_script();
        if let Err(error) = result {
            error!("{}", error);
        }

        info!("Finished deploying Generate");
    }
}
