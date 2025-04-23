use crate::{command::command_func::spawn_command, docker::docker_struct::Container, ssh::ssh_struct::ssh, types::{address::Address, amount::Amount}};

use super::{super::super::common::common_trait::Start, store_struct::Store};
use std::{fs, net::{IpAddr, Ipv4Addr}, path::Path, thread::{self, JoinHandle}};
use log::{debug, info};

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
            if self.config.is_none() && self.db_type.is_some(){
                info!("No config given for database type. Loading default config");
                self.config = Some(self.db_type.unwrap().new());
            }
                
            // Check if node was specified for store
            if let Some(ref node) = self.object.node {
                self.object.remote = Some(node.address.unwrap().clone());
            }
            // Set docker container for store
            let _ = self.object.docker.get_or_insert(Container::new());
            if let Some(ref mut docker) = self.object.docker {
                if let Some(config) = self.config {
                    // Setup the container with needed default parameters for specific [`StoreType`]
                    config.setup_container(docker);
                    
                    if self.schema.has_something() {
                        config.mount_data(self.schema, docker);
                    }
                    // Upload all sql files of non was specified
                    else if let Some(ref data) = self.object.data {
                        let path = fs::read_dir(data).unwrap();
                        let mut sql_files = Vec::<String>::new();
                        for file in path {
                            if let Ok(file) = file {
                                let file = file.path();
                                let is_sql_file = file.to_str().unwrap().contains(".sql");
                                if is_sql_file {
                                    sql_files.push(file.display().to_string());
                                }
                            }
                        }
                        config.mount_data(Amount::Multiple(sql_files), docker);
                    }
                }
                            
                // Start docker container
                let _ = docker.build();

                self.object.ssh = Some(docker.get_ssh());
                self.object.remote = Some(Address { ip: IpAddr::V4(Ipv4Addr::LOCALHOST), port: docker.get_ssh_port().unwrap().clone() } );
            }

            let remote = self.object.remote.unwrap();
            
            let _ = spawn_command(&format!("ansible-playbook src/ansible/ansible-setup.yml -e \"port={}\"", remote.port)).wait();
           
            // Upload data directory
            let upload_directory = self.object.upload_data();

            debug!("upload dir : {:?}", upload_directory);
            // Run start script
            if let Some(ref mut start) = self.object.start {
                if start.contains("sh") {
                    self.object.ssh.unwrap().exec(format!("sh /{}", start.strip_prefix(Path::new(&start).parent().unwrap().parent().unwrap().to_str().unwrap()).unwrap()));
                }
            }
            else {
                self.object.ssh.unwrap().exec(format!("sh /{}/setup.sh", upload_directory));
            }

            info!("Finished");
        }).unwrap()
    }
}

