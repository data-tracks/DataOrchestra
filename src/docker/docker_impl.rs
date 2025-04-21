use std::collections::HashMap;
use std::fmt::format;
use std::net::{IpAddr, Ipv4Addr};
use std::process::{Child, Command, Stdio};
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use std::u16;
use log::{debug, info, warn};
use crate::docker::docker_struct::Container;
use crate::command::command_func::{output_command, spawn_command, status_command};
use crate::ssh::ssh_struct::ssh;
use crate::types::amount::Amount;

use super::docker_struct::{default_address, default_compose, default_file, default_image, default_mount, default_name, default_network, default_options};

/// Factory for the creation of a docker container 
impl Container {
    /// Set name of docker container
    pub fn set_name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Add network to docker container
    pub fn set_network<T: Into<String>>(mut self, network: T) -> Self {
        self.network = network.into();
        self
    }

    /// Add image to docker container
    pub fn set_image<T: Into<String>>(mut self, image: T) -> Self {
        self.image = Some(image.into());
        self
    }

    /// Add enviroment variables to docker container
    pub fn add_env_var<T: Into<String>, S: Into<String>>(mut self, key: T, value: S) -> Self {
        if let Some(ref mut map) = self.options {
            map.insert(key.into(), value.into());
        }
        else {
            let mut map = HashMap::<String,String>::new();
            map.insert(key.into(), value.into());
            self.options = Some(map);
        }

        self
    }

    /// Add a directory mount to docker container
    pub fn add_mount<T: Into<String>, S: Into<String>>(mut self, mount: T, target: S) -> Self {
        let mount_value = format!("{}:{}", mount.into(), target.into());
        if let Some(ref mut amount) = self.mount {
            if let Amount::Single(value) = amount {
                let array = vec![value.clone(), mount_value];
                self.mount = Some(Amount::Multiple(array));
            }
            else if let Amount::Multiple(ref mut values) = amount {
                values.push(mount_value);
            } 
        }
        else {
            self.mount = Some(Amount::Single(mount_value));
        }
        self
    }
    
    pub fn set_compose<T: Into<String>>(mut self, compose: T) -> Self {
        self.compose = Some(compose.into());
        self
    }
}

impl Container {
    /// Get [`Container`] with filled with default values
    pub fn new() -> Self {
        Container {
            name: default_name(),
            address: default_address(),
            image: default_image(),
            network: default_network(),
            mount: default_mount(),
            options: default_options(),
            compose: default_compose(),
            file: default_file(),
            id: None, 
            ssh_port: None
        }
    }

    /// Initialise docker container based on specified data in [`Container`] struct.
    pub fn init(&mut self) -> Result<(), String>{
        // Hierarchy creation. compose > file > image
        if let Some(ref compose) = self.compose {
            info!("Initializing docker container from docker compose");
            let _ = spawn_command(format!("docker compose -f {} up -d --build", compose)).wait();
        }
        else {
            info!("Initializing docker container from docker image");

            // Create network
            let networks: String = output_command("docker network ls");
            if !networks.contains(&self.network) {
                debug!("{}", format!("Creating network bridge {}", &self.network));
                let create_bridge = spawn_command(&format!("docker network create -d bridge {}", &self.network))
                    .wait();

                if create_bridge.is_err() || (create_bridge.is_ok() && !&create_bridge.as_ref().unwrap().success()) {
                    warn!("Unable to create bridge {} | Code : {}", &self.network, &create_bridge.unwrap().code().unwrap());
                }
                else {
                    debug!("Successfully created bridge [{}]", &self.network)
                }
            }
            
            // Create image
            let id = output_command(&format!("docker run {}", self.get_options()));
            debug!("id : {}", &id);
            self.id = Some(id.trim().to_string());
        }
        // get ip
        let ip_output = output_command(format!("docker inspect -f {{{{range.NetworkSettings.Networks}}}}{{{{.IPAddress}}}}{{{{end}}}} {}", self.id.as_ref().unwrap()));
        let ip_output = ip_output.replace("\n", "");
        let ip = Ipv4Addr::from_str(ip_output.as_str());
        if let Ok(ip) = ip {
            self.address.ip = IpAddr::V4(ip);
            debug!("ip: {}", self.address.ip);
        }
        else {
            panic!("Couldn't get ip from docker container {}", self.address.ip);
        }

        // Get ssh port
        let ports = output_command(format!("docker port {}", self.id.as_ref().unwrap()));
        for port in ports.split("\n") {
            if port.contains("22") {
                self.ssh_port = Some(port.split(":").last().unwrap().parse::<u16>().unwrap());
                break;
            }
        }
        debug!("ssh port : {}", self.ssh_port.unwrap());

        // Install ssh server
        info!("Installing shh server on {}", &self.id.as_ref().unwrap());
        // Reformat sh script for linux distro
        if cfg!(target_os = "windows") {
            spawn_command(&"dos2unix src/docker/docker_ssh_init.sh".to_string());
        }

        let _ = spawn_command(&format!("docker cp src/docker/docker_ssh_init.sh {}:/", &self.id.as_ref().unwrap())).wait();
        let _ = status_command(&format!("docker exec {} sh ../docker_ssh_init.sh", &self.id.as_ref().unwrap()));
        // Start ssh server
        let _ = spawn_command(&format!("docker exec -d {} /usr/sbin/sshd -D", &self.id.as_ref().unwrap())).wait();

        // Sleep to wait for ssh server to properly start
    sleep(Duration::from_secs(1));
    
        Ok(())
    }

    /// Get docker container options as command input
    pub fn get_options(&self) -> String {
        let mut command: String = String::from("-d -q");

        command = format!("{command} --network={}", &self.network);

        if let Some(ref name) = self.name {
            command = format!("{command} --name={}", name);
        }
        // Publish ssh port
        command = format!("{command} -p 22");
       
        // Publish postgres port
        if self.image.as_ref().unwrap() == "postgres" {
            command = format!("{command} -p 5432:5432");
        }

        if let Some(options) = &self.options {
            for (key, value) in options {
                command = format!("{command} -e {key}={value}")
            }
        }

        if let Some(source) = &self.mount {
            match source {
                Amount::None => (),
                Amount::Single(value ) => command = format!("{command} -v {}", value),
                Amount::Multiple(values) => {
                    for value in values {
                        command = format!("{command} -v {}", value);
                    }
                }
            }
        }

        command = format!("{command} -it {}", self.image.as_ref().unwrap());

        command
    }

    /// Get ssh connection to docker container
    pub fn get_ssh(&mut self) -> ssh {
        let mut ssh = ssh::new();
        ssh.connect(&"127.0.0.1".to_string(), self.ssh_port.unwrap(), &"root".to_string(), &"password".to_string());

        ssh
    }
    /// Execute command remotely in docker container
    ///
    /// # Examples
    /// 
    /// ```
    /// use DataOrchestra::docker::docker_struct::Container;
    /// let docker: Container =  { image: "ubuntu" };
    /// docker.execute("pwd");
    /// ```
    fn execute<T: Into<String>>(&self, arg: T) -> Child {
        let command = format!("docker exec {} {}", &self.name.as_ref().unwrap(), &arg.into());
        debug!("{}", format!("Running command: {}", command));
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .arg("/C")
                .arg(command)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(command)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("failed to execute process")
        };

        output    
    }
}
