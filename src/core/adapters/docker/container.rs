use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::process::{Child, Command, Stdio};
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use log::{debug, info, error};
use serde::de;
use crate::core::adapters::command::command_func::{output_command, spawn_command, status_command};
use crate::core::adapters::docker::create_network;
use crate::shared::Amount;
use crate::core::adapters::ssh::Ssh;

use super::traits::EnvBuilder;

/// The docker `ContainerData` type. Represents the general information tied to the creation of a
/// docker container.
#[derive(Debug)]
pub struct Container {
    pub image: Option<String>,
    pub dockerfile: Option<String>,
    // Additional building args for dockerfile
    pub build_args: HashMap<String, String>,

    // Name of container
    pub name: Option<String>,
    // Network container belongs to
    pub network: String,
    // Additional arguments for image
    pub options: HashMap<String, String>,
    // Mounting values of data
    pub mount: Vec<String>,
        // Publish all ports
    pub publish_all: bool,
    /// Id of container
    pub id: Option<String>,
    /// Ip of container
    pub ip: Option<IpAddr>,
    /// Published ports of container. A vector of [`PortMap`] which defines the combination
    /// `host:internal`.
    pub publish_ports: Vec<PortMap>,
    pub is_running: bool,
}

impl Default for Container {
    fn default() -> Self {
        Container {
            image: None,
            dockerfile: None,
            name: None,
            network: String::from("orchestra"),
            mount: Vec::new(),
            options: HashMap::new(),
            build_args: HashMap::new(),
            publish_all: false,
            id: None,
            ip: None,
            publish_ports: Vec::<PortMap>::new(),
            is_running: false
        }
    }
}

#[derive(Debug)]
pub struct PortMap {
    host: u16,
    internal: u16
}

impl PortMap {
    pub fn new(host: u16, internal: u16) -> Self {
        PortMap { host, internal }
    }

    pub fn get_host(&self) -> u16 {
        self.host.clone()
    }

    pub fn get_internal(&self) -> u16 {
        self.internal.clone()
    }
}

// Public methods for docker container 
impl Container {
    pub fn new(
            image: Option<String>,
            dockerfile: Option<String>,
            build_args: Option<HashMap<String, String>>,
            name: Option<String>,
            network: Option<String>,
            options: Option<HashMap<String, String>>,
            mount: Option<Vec<String>>,
            publish_all: bool
        ) -> Self {
        let default = Container::default();
        Container
        {
            image: image.or(default.image),
            dockerfile: dockerfile.or(default.dockerfile),
            build_args: build_args.or(Some(default.build_args)).unwrap(),
            name: name.or(default.name),
            network: network.or(Some(default.network)).unwrap(),
            options: options.or(Some(default.options)).unwrap(),
            mount: mount.or(Some(default.mount)).unwrap(),
            publish_all,
            id: default.id,
            ip: default.ip,
            publish_ports: default.publish_ports,
            is_running: default.is_running 
        }
    }

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

    pub fn set_publish_all(&mut self, value: bool) -> &mut Self {
        self.publish_all = value;
        self
    }
    
    /// Add enviroment variables to docker container
    pub fn add_env_var<T: Into<String>, S: Into<String>>(&mut self, key: T, value: S) -> &mut Self {
        self.options.insert(key.into(), value.into());
        self
    }

    /// Add building arguments for dockerfile
    pub fn add_build_arg<T: Into<String>, S: Into<String>>(&mut self, key: T, value: S) -> &mut Self {
        self.options.insert(key.into(), value.into());
        self
    }

    /// Add a directory mount to docker container
    pub fn add_mount<T: Into<String>, S: Into<String>>(&mut self, mount: T, target: S) -> &mut Self {
        let mount_value = format!("{}:{}", mount.into(), target.into());
        self.mount.push(mount_value);
        self
    }

    /// Set id of docker container
    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    /// Get id of docker container
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Add [`PortMap`] to docker container
    pub fn add_port_map(&mut self, host: u16, internal: u16) {
        let map = PortMap::new(host, internal);
        self.publish_ports.push(map);
    }

    pub fn set_image<T: Into<String>>(&mut self, image: T) -> &mut Self {
        self.image = Some(image.into());
        self
    }

    pub fn set_dockerfile<T: Into<String>>(&mut self, dockerfile: T) -> &mut Self {
        self.dockerfile = Some(dockerfile.into());
        self
    }
}

impl Container {
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
        }

        for (key, value) in &self.options {
            command = format!("{command} -e {key}={value}")
        }

        // Parse mount 
        for value in &self.mount {
            command = format!("{command} -v {}", value);
        }

        command
    }

    /// Get ssh connection to docker container
    pub fn get_ssh(&mut self) -> Ssh {
        let mut ssh = Ssh::new();
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
        let ip = output_command(format!("docker inspect -f {{{{range.NetworkSettings.Networks}}}}{{{{.IPAddress}}}}{{{{end}}}} {}", self.id.as_ref().unwrap()));
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
        for portmap in &self.publish_ports {
            if portmap.get_host() == host {
                return Ok(portmap.get_internal());
            }
        };

        return Err(String::from("Unable to find given host port"));
    }
   
    /// Get the host port mapped to the `internal` port
    pub fn get_external_port(&self, internal: u16) -> Result<u16, String> {
        for portmap in &self.publish_ports {
            if portmap.get_internal() == internal {
                return Ok(portmap.get_host());
            }
        };

        return Err(String::from("Unable to find given internal port"));
    }
}

impl EnvBuilder<(), String> for Container {
    /// validate if the container is buildable
    fn validate(&self) -> bool {
       true 
    }

    /// Create docker container using a dockerfile or image
    fn build(&mut self) -> Result<(), String> {
        if let Some(ref dockerfile) = self.dockerfile {
            let mut build_args_string = String::new();
            for (key, value) in self.build_args.iter() {
                build_args_string = format!("{build_args_string} {}={}", key, value);
            }

            if build_args_string.is_empty() {
                let _ = output_command(format!("docker build -t {} {}", self.image.as_ref().unwrap(), dockerfile));
            }
            else {
                let _ = output_command(format!("docker build --build-arg {} -t {} {}", build_args_string, self.image.as_ref().unwrap(), dockerfile));
            }
        }

        // Create network
        let network = create_network(self.network.clone());
        match network {
            Ok(_) => info!("Successfully created network"),
            Err(value) => error!("{}", value),
        }

        // Create image
        let id = output_command(format!("docker run {} -it {}", self.get_options(), self.image.as_ref().unwrap()));
        self.set_id(id.trim().to_string());

        // get ip
        let ip = self.get_ip();
        match ip {
            Ok(ip) => self.ip = Some(ip),
            Err(error) => error!("{}", error)
        }

        // Get ssh port
        let ports = output_command(format!("docker port {}", self.id.as_ref().unwrap()));
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

        let _ = spawn_command(&format!("docker cp src/docker/docker_ssh_init.sh {}:/", self.id.as_ref().unwrap())).wait();
        let _ = status_command(&format!("docker exec {} sh ../docker_ssh_init.sh", self.id.as_ref().unwrap()));
        // Start ssh server
        let _ = spawn_command(&format!("docker exec -d {} /usr/sbin/sshd -D", self.id.as_ref().unwrap())).wait();

        // Sleep to wait for ssh server to properly start
        sleep(Duration::from_secs(1));

        Ok(())     
    }
}
