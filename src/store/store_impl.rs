use crate::{command::command_func::spawn_command, docker::docker_struct::Container, ssh::ssh_struct::ssh, types::amount::Amount};

use super::{super::common::common_trait::Start, store_struct::Store};
use core::error;
use std::{fs, path::Path, thread::{self, JoinHandle}};
use log::{debug, error, info};

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

            dbg!("{:?}", &self);
            // Create default config of specified database type ([`StoreType`]) if none was
            // specified
            if self.config.is_none() && self.db_type.is_some(){
                info!("No config given for database type. Loading default config");
                self.config = Some(self.db_type.unwrap().new());
            }
                
            // Check if node was specified for store
            if let Some(node) = self.object.node {
                self.object.remote = Some(node.address.unwrap().clone());
            }
            // Set docker container for store
            let _ = self.object.docker.get_or_insert(Container::new());
            if let Some(mut docker) = self.object.docker {
                if let Some(config) = self.config {
                    // Setup the container with needed default parameters for specific [`StoreType`]
                    docker = config.setup_container(docker);

                    if let Some(schema) = self.schema {
                            docker = config.mount_data(schema, docker);
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
                        docker = config.mount_data(Amount::Multiple(sql_files), docker);
                    }
                }
                            
                // Start docker container
                let _ = docker.init();

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
            let remote = self.object.remote.unwrap(); //self.object.get_remote_connection();
            
            let _ = spawn_command(&format!("ansible-playbook src/ansible/ansible-setup.yml -e \"port={}\"", remote.port)).wait();
           
            // Upload data directory
            let mut upload_directory = String::from("/");
            if let Some(ref data) = self.object.data {
                let upload = ssh.upload_directory(&Path::new(&data), &Path::new("/"));
                if let Ok(dir) = upload {
                    upload_directory = dir
                }
            }

            debug!("upload dir : {:?}", upload_directory);
            // Run start script
            if let Some(ref mut start) = self.object.start {
                if start.contains("sh") {
                    ssh.exec(format!("sh /{}", start.strip_prefix(Path::new(&start).parent().unwrap().parent().unwrap().to_str().unwrap()).unwrap()).as_str());
                }
            }
        }).unwrap()
    }
}

