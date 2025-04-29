use std::net::{IpAddr, Ipv4Addr};
use std::thread::{self, JoinHandle};
use std::path::Path;
use log::{info, debug};

use crate::core::adapters::command::command_func::spawn_command;
use crate::core::adapters::docker::{ComposeGroupBuilder, Container, DockerManager};
use crate::core::adapters::ssh::Ssh;
use crate::shared::traits::Start;
use crate::shared::Address;

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
            if self.config.is_none() && self.process_type.is_some() {
                info!("No config given for database type. Loading default config");
                self.config = Some(self.process_type.as_ref().unwrap().new());
            }

            let mut manager = DockerManager::new();

            if let Some(group) = self.object.docker_group_builder.take() {
                debug!("Setting up compose");
                let compose_group = group.build();
                for container in compose_group.containers {
                    manager.add_container(container.config.name.as_ref().unwrap().clone(), container);
                }
            }
            // Take ownership of ContainerBuilder out of object to prevent partial move
            else if let Some(ref process) = self.process_type {
                debug!("Setting up {:?} enviroment", process);
                let mut process_config = &process.new();
                if let Some(ref mut config) = self.config {
                    process_config = config;
                }
                let mut compose = ComposeGroupBuilder::new();
                process_config.setup_container(&mut compose);
                let compose = compose.build();
                for container in compose.containers {
                    manager.add_container(container.config.name.as_ref().unwrap().clone(), container);
                }
            }

            else if let Some(container) = self.object.docker_container_builder.take() {
                debug!("Setting up container");
                let mut container = container.build();
                let _ = container.run();
                manager.add_container(container.config.name.as_ref().unwrap().clone(), container);
            }

            // Run ansible setup script on all containers
            for container in manager.as_vec() {
                let _ = spawn_command(&format!("ansible-playbook src/ansible/ansible-setup.yml -e \"port={}\"", container.get_ssh_port().unwrap())).wait();
            }
            self.object.upload_data();
            
            self.start_script();
            
            info!("Finished");
        }).unwrap()
    }
}

impl Process {
    pub fn start_script(&self) {
        if let Some(ref start) = self.object.start {
            if start.contains("sh") {
                self.object.ssh.as_ref().unwrap().exec(format!("sh /{}", start.strip_prefix(Path::new(&start).parent().unwrap().parent().unwrap().to_str().unwrap()).unwrap()));
            }
        }
        else {
            self.object.ssh.as_ref().unwrap().exec(format!("sh /{}/setup.sh", self.object.upload_directory.as_ref().unwrap()));
        }
    }
}
