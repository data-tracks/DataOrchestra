use crate::{command::command_func::spawn_command, ssh::ssh_struct::ssh};

use super::{super::common::common_trait::Start, store_struct::Store};
use std::{path::Path, thread::{self, JoinHandle}};
use log::info;

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

            if let Some(ref mut docker) = self.docker {
                let _ = docker.init();
                ssh = Some(docker.get_ssh());
                self.remote = Some(docker.address.clone());
            }
          
            if self.remote.is_none() {
                panic!("No remote connection");
            }

            if ssh.is_none() {
                panic!("No ssh connection available");
            }

            let ssh = ssh.unwrap();
            let remote = self.remote.unwrap();
            
            let _ = spawn_command(&format!("ansible-playbook src/ansible/ansible-setup.yml -e \"port={}\"", remote.port)).wait();
            let _ = ssh.upload_directory(&Path::new(&self.script), &Path::new("/"));
            ssh.exec("sh ../store/setup.sh");
        }).unwrap()
    }
}

