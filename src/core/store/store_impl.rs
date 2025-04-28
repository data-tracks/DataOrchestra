use std::fs;
use std::net::{IpAddr, Ipv4Addr};
use std::path::Path;
use std::thread::{self, JoinHandle};
use log::{debug, info};
use crate::core::adapters::command::command_func::spawn_command;
use crate::shared::{traits::Start, Address, Amount};
use crate::core::adapters::ssh::Ssh;

use super::Store;

impl Start<()> for Store {
    /// Start initialisation process for store components
    ///
    /// # Note
    ///
    /// Calling `store.start()` moves the store into the thread
    fn start(mut self) -> JoinHandle<()> {
        info!("Spawning storing thread");
        thread::Builder::new().name("store".to_string()).spawn(move || {
            let mut ssh: Option<Ssh> = None;

            if let Some(ref docker) = self.object.docker {
            }

            // Create default config of specified database type ([`StoreType`]) if none was
            // specified
            if self.config.is_none() && self.db_type.is_some(){
                info!("No config given for database type. Loading default config");
                self.config = Some(self.db_type.unwrap().new());
            }
                
        }).unwrap()
    }
}

impl Store {
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

    pub fn store_container(&mut self) {
        // Set docker container for store
        if let Some(ref mut docker) = self.object.docker {
            if let Some(ref mut config) = self.config {
                container = docker.get_ref_mut_single();
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
            self.object.ssh = Some(docker.config.get_ref_mut_single().get_ssh());
            self.object.remote = Some(Address { ip: IpAddr::V4(Ipv4Addr::LOCALHOST), port: docker.config.get_ref_mut_single().get_ssh_port().unwrap().clone() } );
        }

        let remote = self.object.remote.unwrap();
        
        let _ = spawn_command(&format!("ansible-playbook src/ansible/ansible-setup.yml -e \"port={}\"", remote.port)).wait();
       
        // Upload data directory
        self.object.upload_data();

        // Start script
        self.start_script();
        info!("Finished");
    }


    pub fn store_multicontainer(&mut self) {}
}

