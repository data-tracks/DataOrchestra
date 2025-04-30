use std::fs;
use std::path::Path;
use std::thread::{self, JoinHandle};
use log::{debug, info};
use crate::core::adapters::command::command_func::spawn_command;
use crate::core::adapters::docker::DockerManager;
use crate::core::adapters::ssh::Ssh;
use crate::shared::{traits::Start, Amount};

use super::Store;

impl Start<()> for Store {
    /// Start initialisation process for store components
    ///
    /// # Note
    ///
    /// Calling `store.start()` moves the store into the thread
    fn start(mut self) -> JoinHandle<()> {
        debug!("Spawning storing thread");
        thread::Builder::new().name("store".to_string()).spawn(move || {
            // Create default config of specified database type ([`StoreType`]) if none was
            // specified
            if self.config.is_none() && self.db_type.is_some(){
                info!("No config given for database type. Loading default config");
                self.config = Some(self.db_type.as_ref().unwrap().new());
            }

            // Create and run containers
            let mut manager = DockerManager::new();

            // Take ownership of ComposeGroupBuilder out of object to prevent partial move 
            if let Some(group) = self.object.docker_group_builder.take() {
                debug!("Setting up compose");
                let compose_group = group.build();
                for container in compose_group.containers {
                    manager.add_container(container.config.name.as_ref().unwrap().clone(), container);
                }
            }  

            // Take ownership of ContainerBuilder out of object to prevent partial move
            else if let Some(mut container) = self.object.docker_container_builder.take() {
                debug!("Setting up container");
                if let Some(ref database) = self.db_type {
                    let mut db_config = &database.new();
                    if let Some(ref mut config) = self.config {
                        db_config = config;
                    }
                    db_config.setup_container(&mut container); 

                    if self.schema.len() > 0 {
                        //TODO: Maybe remove clone
                        db_config.mount_data(Amount::Multiple(self.schema.clone()), &mut container);
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
                        db_config.mount_data(Amount::Multiple(sql_files), &mut container);
                    }
                }

                let mut container = container.build();
                let _ = container.run();
                manager.add_container(container.config.name.as_ref().unwrap().clone(), container);
            }

            // Run ansible setup script on all containers
            for container in manager.as_vec() {
                let _ = spawn_command(&format!("ansible-playbook scripts/ansible/ansible-setup.yml -e \"port={}\"", container.get_ssh_port().unwrap())).wait();
            }

            // Upload data directory
            self.object.upload_data();

            // Start script
            self.start_script();
            info!("Finished");
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

    pub fn start_script_<T: Into<String>>(path: T, ssh: &Ssh) {
        let path = path.into();
        if path.ends_with(".sh") {
            ssh.exec(format!("sh /{}", path));
        }
    }
}

