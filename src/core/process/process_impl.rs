use std::thread::{self, JoinHandle};
use log::{info, debug};
use crate::core::adapters::command::command_func::spawn_command;
use crate::core::adapters::docker::{ComposeGroupBuilder, DockerManager};
use crate::core::utils::start_script;
use crate::shared::traits::Start;
use crate::core::adapters::docker::Run;

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
                debug!("Setting up compose");
                let mut compose_group = group.build();
                let result = compose_group.run();
                for container in compose_group.containers {
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

            for (container, data) in manager.iter_combine_data(&self.object.data) {
                if let Some(ref ssh) = container.ssh {
                    let _ = ssh.upload_directory(&data.path, &data.destination);
                }
                else {
                    panic!("Ssh client unavailable");
                }
            }

            for (container, data) in manager.iter_combine_data(&self.object.data) {
                if let Some(ref ssh) = container.ssh {
                    start_script(ssh, data);
                }
                else {
                    panic!("Ssh client unavailable");
                }
            }
            
            
            info!("Finished");
        }).unwrap()
    }
}
