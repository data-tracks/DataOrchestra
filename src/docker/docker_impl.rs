use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::process::{Child, Command, Stdio};
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use std::{default, u16};
use log::{debug, error, info};
use crate::docker::create_network;
use crate::docker::docker_struct::{Container, PortMap};
use crate::command::command_func::{output_command, spawn_command, status_command};
use crate::ssh::ssh_struct::ssh;
use crate::types::amount::Amount;

use super::docker_struct::{default_build_args, default_compose, default_file, default_image, default_mount, default_name, default_network, default_options, default_publish_all, Meta};

// Public methods for docker container 
impl Container {
    /// Set name of docker container
    pub fn set_name<T: Into<String>>(&mut self, name: T) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    /// Add network to docker container
    pub fn set_network<T: Into<String>>(&mut self, network: T) -> &mut Self {
        self.network = network.into();
        self
    }

    /// Add image to docker container
    pub fn set_image<T: Into<String>>(&mut self, image: T) -> &mut Self {
        self.image = Some(image.into());
        self
    }
    
    pub fn set_compose<T: Into<String>>(&mut self, compose: T) -> &mut Self {
        self.compose = Some(compose.into());
        self
    }

    pub fn set_publish_all(&mut self, value: bool) -> &mut Self {
        self.publish_all = value;
        self
    }
    
    /// Add enviroment variables to docker container
    pub fn add_env_var<T: Into<String>, S: Into<String>>(&mut self, key: T, value: S) -> &mut Self {
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

    pub fn add_build_arg<T: Into<String>, S: Into<String>>(&mut self, key: T, value: S) -> &mut Self {
        if let Some(ref mut map) = self.build_args {
            map.insert(key.into(), value.into());
        }
        else {
            let mut map = HashMap::<String,String>::new();
            map.insert(key.into(), value.into());
            self.build_args = Some(map);
        }

        self
 
    }

    /// Add a directory mount to docker container
    pub fn add_mount<T: Into<String>, S: Into<String>>(&mut self, mount: T, target: S) -> &mut Self {
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

    pub fn set_id(&mut self, id: String) {
        self.meta.id = Some(id);
    }

    pub fn get_id(&self) -> Option<&String> {
        self.meta.id.as_ref()
    }

    pub fn add_port_map(&mut self, host: u16, internal: u16) {
        let map = PortMap::new(host, internal);
        if let Some(ref mut ports) = self.meta.publish_ports {
            ports.push(map);
        }
        else {
            self.meta.publish_ports = Some(vec![map]);
        }
    }

}

impl Container {
    /// Get [`Container`] with filled with default values
    pub fn new() -> Self {
        Container {
            name: default_name(),
            image: default_image(),
            network: default_network(),
            mount: default_mount(),
            build_args: default_build_args(),
            publish_all: default_publish_all(),
            options: default_options(),
            compose: default_compose(),
            file: default_file(),
            meta: Meta::default(),
        }
    }

    /// Initialise docker container based on specified data in [`Container`] struct.
    pub fn build(&mut self) -> Result<(), String>{
        // Hierarchy creation. compose > file > image
        if let Some(ref compose) = self.compose {
            info!("Initializing docker container from docker compose");
            let _ = spawn_command(format!("docker compose -f {} up -d --build", compose)).wait();
        }
        else {
            info!("Initializing docker container from docker image");

            // Create network
            let network = create_network(&self.network);
            match network {
                Ok(_) => info!("Successfully created network"),
                Err(value) => error!("{}", value),
            }

            // Build dockerfile 
            if let Some(ref file) = self.file {
                if let Some(ref image) = self.image {
                    let mut build_args = String::new();
                    if let Some(ref args) = self.build_args {
                        for (key, value) in args.iter() {
                            build_args = format!(" {}={}", key, value);
                        }
                    }

                    if build_args.is_empty() {
                        let _ = output_command(format!("docker build -t {} {}", image, file));
                    }
                    else {
                        let _ = output_command(format!("docker build --build-arg {} -t {} {}", build_args, image, file));
                    }
                }
                else {
                    panic!("Please provide image along with the dockerfile such that the dockerfile can be properly build");
                }
            }

            // Create image
            let id = output_command(format!("docker run {}", self.get_options()));
            self.set_id(id.trim().to_string());
        }

        // get ip
        let ip = self.get_ip();
        match ip {
            Ok(ip) => self.meta.ip = Some(ip),
            Err(error) => error!("{}", error)
        }

        // Get ssh port
        let ports = output_command(format!("docker port {}", self.meta.id.as_ref().unwrap()));
        for port in ports.split("\n").filter(|x| !x.is_empty()) {
            let (int, ext) = port.split_once("/").unwrap();
            let int = int.parse::<u16>().unwrap();
            let ext = ext.split(":").last().unwrap().parse::<u16>().unwrap();
            self.add_port_map(ext, int);
        }

        // Install ssh server
        info!("Installing shh server on {}", self.get_id().unwrap());
        // Reformat sh script for linux distro
        if cfg!(target_os = "windows") {
            spawn_command(&"dos2unix src/docker/docker_ssh_init.sh".to_string());
        }

        let _ = spawn_command(&format!("docker cp src/docker/docker_ssh_init.sh {}:/", &self.meta.id.as_ref().unwrap())).wait();
        let _ = status_command(&format!("docker exec {} sh ../docker_ssh_init.sh", &self.meta.id.as_ref().unwrap()));
        // Start ssh server
        let _ = spawn_command(&format!("docker exec -d {} /usr/sbin/sshd -D", &self.meta.id.as_ref().unwrap())).wait();

        // Sleep to wait for ssh server to properly start
        sleep(Duration::from_secs(1));
    
        Ok(())
    }

    /// Get docker container options as command input
    pub fn get_options(&self) -> String {
        let mut command: String = String::from("-d -q");

        // Parse network variable
        command = format!("{command} --network={}", &self.network);

        // Parse name variable
        if let Some(ref name) = self.name {
            command = format!("{command} --name={}", name);
        }
    
        if self.publish_all {
            command = format!("{command} -P");
        }
        else {
            // Publish ssh port
            command = format!("{command} -p 22");
           
            // Publish postgres port
            if self.image.as_ref().unwrap() == "postgres" {
                command = format!("{command} -p 5432");
            }
        }

        // Parse enviroment variables
        if let Some(options) = &self.options {
            for (key, value) in options {
                command = format!("{command} -e {key}={value}")
            }
        }

        // Parse mount 
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

        // Parse image variable
        command = format!("{command} -it {}", self.image.as_ref().unwrap());

        command
    }

    /// Get ssh connection to docker container
    pub fn get_ssh(&mut self) -> ssh {
        let mut ssh = ssh::new();
        ssh.connect(&"127.0.0.1".to_string(), self.get_ssh_port().unwrap(), &"root".to_string(), &"password".to_string());

        ssh
    }

    /// Execute command remotely in docker container
    ///
    /// # Examples
    /// 
    /// ```
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


    /// Get ip of docker container
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub fn get_ip(&self) -> Result<IpAddr, String>  {
        let ip = output_command(format!("docker inspect -f {{{{range.NetworkSettings.Networks}}}}{{{{.IPAddress}}}}{{{{end}}}} {}", self.meta.id.as_ref().unwrap()));
        let ip = ip.replace("\n", "").trim().to_string();
        let ip = Ipv4Addr::from_str(ip.as_str());
        if let Err(err) = ip {
            return Err(err.to_string());
        }
        
        Ok(IpAddr::V4(ip.unwrap()))
    }

    /// Get host ssh port mapping from docker container
    pub fn get_ssh_port(&self) -> Result<u16, String> {
        self.get_external_port(22)     
    }

    /// Get the internal port mapped to the `host` port
    pub fn get_internal_port(&self, host: u16) -> Result<u16, String> {
        if let Some(ref ports) = self.meta.publish_ports {
            for portmap in ports {
                if portmap.get_host() == host {
                    return Ok(portmap.get_internal());
                }
            };
        }
        else {
            return Err(String::from("No published ports available"));
        }

        return Err(String::from("Unable to find given host port"));
    }
   
    /// Get the host port mapped to the `internal` port
    pub fn get_external_port(&self, internal: u16) -> Result<u16, String> {
        if let Some(ref ports) = self.meta.publish_ports {
            for portmap in ports {
                if portmap.get_internal() == internal {
                    return Ok(portmap.get_host());
                }
            };
        }
        else {
            return Err(String::from("No published ports available"));
        }

        return Err(String::from("Unable to find given internal port"));
    }
}


// Private methods docker container 
impl Container {
    }
