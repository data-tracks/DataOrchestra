use crate::{command::command_func::spawn_command, ssh::ssh_struct::ssh, store::store_types::{MySQL, PostGres, Redis, StoreData, StoreType, StoreTypeConfig}, types::amount::Amount};

use super::{super::common::common_trait::Start, store_struct::Store};
use std::{path::Path, thread::{self, JoinHandle}};
use log::{info, warn};

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
            
            if self.config.is_none() {
               match self.db_type {
                    StoreType::PostGres => self.config = Some(StoreTypeConfig::PostGres(PostGres::new())),
                    StoreType::Redis => self.config = Some(StoreTypeConfig::Redis(Redis::new())),
                    StoreType::MySQL => self.config = Some(StoreTypeConfig::MySQL(MySQL::new())),
                    _ => warn!("Unrecoginesed store type provided")
               } 
            }
                
            if let None = self.config {
                warn!("No config available for the store type");
            }

            let config = self.config.as_ref().unwrap();
            
            self.docker = self.docker
                .set_image(config.get_image());

            self.docker = self.config.setup_container(self.docker);


            if let StoreTypeConfig::PostGres(postgres) = config {
                self.docker = self.docker
                    .add_env_var(String::from("POSTGRES_DB"), postgres.postgres_db.clone())
                    .add_env_var(String::from("POSTGRES_USER"), postgres.postgres_user.clone())
                    .add_env_var(String::from("POSTGRES_PASSWORD"), postgres.postgres_password.clone());

                if let Some(ref schema) = self.schema {
                    match schema {
                        Amount::Single(value) => self.docker = self.docker.add_mount(value, &String::from("/docker-entrypoint-initdb.d/")),
                        Amount::Multiple(values) => {
                            for value in values {
                                self.docker = self.docker.add_mount(value, &String::from("/docker-entrypoint-initdb.d/"));
                            }
                        }
                    }
                }
            }


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
            let _ = ssh.upload_directory(&Path::new(&self.script), &Path::new("/"));

            if self.script.contains("sh") {
                ssh.exec(format!("sh {}", self.script).as_str());
            }
        }).unwrap()
    }
}

