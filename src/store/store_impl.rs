use crate::{command::command_func::spawn_command, ssh::ssh_struct::ssh};

use super::{super::common::common_trait::Start, store_struct::Store};
use std::{path::Path, thread::{self, JoinHandle}};
use log::{debug, info, warn};

impl Start<()> for Store {
    /// Start initialisation process for store components
    ///
    /// # Note
    ///
    /// Calling `store.start()` moves the store into the thread
    fn start(mut self) -> JoinHandle<()> {
        info!("Spawning storing thread");
        thread::Builder::new().name("store".to_string()).spawn(move || {
            let mut ssh: Option<ssh> = None;

            // Create default config of specified database type ([`StoreType`]) if none was
            // specified
            if self.config.is_none() {
                self.config = Some(self.db_type.new());
            }
                
            if let None = self.config {
                warn!("No config available for the store type");
            }

            // Setup the container with needed default parameters for specific [`StoreType`]
            let config = self.config.as_mut().unwrap();
            self.docker = config.setup_container(self.docker);
            if let Some(schema) = self.schema {
                self.docker = config.mount_data(schema, self.docker);
            }
            
            // Start docker container
            let _ = self.docker.init();

            ssh = Some(self.docker.get_ssh());
            self.remote = Some(self.docker.address.clone());
            
            if self.remote.is_none() {
                panic!("No remote connection");
            }

            if ssh.is_none() {
                panic!("No ssh connection available");
            }

            let ssh = ssh.unwrap();
            let remote = self.remote.unwrap();
            
            let _ = spawn_command(&format!("ansible-playbook src/ansible/ansible-setup.yml -e \"port={}\"", remote.port)).wait();
            let upload_directory = ssh.upload_directory(&Path::new(&self.data), &Path::new("/"));
            debug!("upload dir : {:?}", upload_directory);
            // Run start script
            if self.start_script.contains("sh") {
                ssh.exec(format!("sh /{}", self.start_script.strip_prefix(Path::new(&self.start_script).parent().unwrap().parent().unwrap().to_str().unwrap()).unwrap()).as_str());
            }
        }).unwrap()
    }
}

