use crate::{command::command_func::spawn_command, ssh::ssh_struct::ssh, types::address::Address};

use super::{super::super::common::common_trait::Start, generate_struct::Generate};
use std::{net::{IpAddr, Ipv4Addr}, path::Path, thread::{self, JoinHandle}};
use log::{debug, info};

impl Start<()> for Generate {
    /// Start initialisation process for store components
    ///
    /// # Note
    ///
    /// Calling `generate.start()` moves the store into the thread
    fn start(mut self) -> JoinHandle<()> {
        info!("Spawning generate thread");
        
        thread::Builder::new().name("generate".to_string()).spawn(move || {
            let mut ssh: Option<ssh> = None;

            if let Some(ref mut docker) = self.docker {
                let _ = docker.build();
                ssh = Some(docker.get_ssh());
                self.remote = Some( Address { ip: IpAddr::V4(Ipv4Addr::LOCALHOST), port: docker.get_ssh_port().unwrap().clone() } );
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
            else {
                self.object.ssh.unwrap().exec(format!("sh /{}/setup.sh", upload_directory));
            }


            info!("Finished");
        }).unwrap()
    }
}
