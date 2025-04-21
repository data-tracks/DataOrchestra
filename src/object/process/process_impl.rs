use crate::{command::command_func::spawn_command, docker::docker_struct::Container, ssh::ssh_struct::ssh};

use super::{super::super::common::common_trait::Start, process_struct::Process};
use std::{path::Path, thread::{self, JoinHandle}};
use log::{info, debug};

impl Start<()> for Process {
    /// Start initialisation process for store components
    ///
    /// # Note
    ///
    /// Calling `process.start()` moves the store into the thread
    fn start(mut self) -> JoinHandle<()> {
        info!("Spawning generate thread");
        
        thread::Builder::new().name("process".to_string()).spawn(move || {
            let mut ssh: Option<ssh> = None;

            if self.config.is_none() {
                info!("No config given for database type. Loading default config");
                self.config = Some(self.process_type.unwrap().new());
            }
 
            let _ = self.object.docker.get_or_insert(Container::new());
            if let Some(mut docker) = self.object.docker {
                let config = self.config.as_mut().unwrap();
                // Setup the container with needed default parameters for specific [`StoreType`]
                docker = config.setup_container(docker);
 
                let _ = docker.build();
                ssh = Some(docker.get_ssh());
                self.object.remote = Some(docker.address.clone());
            }
            
            if self.object.remote.is_none() {
                panic!("No remote connection");
            }

            if ssh.is_none() {
                panic!("No ssh connection available");
            }

            let ssh = ssh.unwrap();
            let remote = self.object.remote.unwrap();
            
            let _ = spawn_command(&format!("ansible-playbook src/ansible/ansible-setup.yml -e \"port={}\"", remote.port)).wait();

            let mut upload_directory = String::from("/");
            if let Some(ref data) = self.object.data {
                let upload = ssh.upload_directory(&Path::new(data), &Path::new("/"));
                if let Ok(dir) = upload {
                    upload_directory = dir
                }
            }

            debug!("upload dir : {:?}", upload_directory);
            if let Some(ref mut start) = self.object.start {
            // Run start script
                if start.contains("sh") {
                    ssh.exec(format!("sh /{}", start.strip_prefix(Path::new(&start).parent().unwrap().parent().unwrap().to_str().unwrap()).unwrap()).as_str());
                }
            }
        }).unwrap()
    }
}
