use std::net::{IpAddr, Ipv4Addr};
use std::process::{Child, Command, Stdio};
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use log::{debug, info, error};
use crate::core::adapters::command::command_func::{output_command, spawn_command, status_command};
use crate::core::adapters::docker::create_network;
use crate::core::adapters::ssh::Ssh;

use super::config::ContainerConfigBuilder;
use super::source::DockerSourceBuilder;
use super::{ContainerConfig, DockerSource, PortMapping};
use super::Run;

/// The docker `ContainerData` type. Represents the general information tied to the creation of a
/// docker container.
#[derive(Debug)]
pub struct Container {
    /// Id of container
    pub id: Option<String>,
    /// Ip of container
    pub ip: Option<IpAddr>,
    /// Published ports of container. A vector of [`PortMap`] which defines the combination
    /// `host:internal` or `external:internal`.
    pub publish_ports: Vec<PortMapping>,
    /// If container is available and running
    pub is_running: bool,
    /// Ssh client connected to container
    pub ssh: Option<Ssh>,
    /// Container config
    pub config: ContainerConfig,
    /// Container creation source
    pub source: DockerSource,
}

#[derive(Debug)]
pub struct ContainerBuilder {
    containerconfig: ContainerConfigBuilder,
    dockersource: DockerSourceBuilder,
}

impl ContainerBuilder {
    pub fn new() -> ContainerBuilder {
        ContainerBuilder { containerconfig: ContainerConfigBuilder::new(), dockersource: DockerSourceBuilder::new() }
    }

    pub fn set_name<T: Into<String>>(&mut self, name: T) -> &mut Self {
        self.containerconfig.set_name(name);
        self
    }

    pub fn set_network<T: Into<String>>(&mut self, network: T) -> &mut Self {
        self.containerconfig.set_network(network);
        self
    }

    pub fn add_env_var<T: Into<String>, S: Into<String>>(&mut self, key: T, value: S) -> &mut Self {
        self.containerconfig.add_env_var(key, value);
        self
    }

    pub fn add_mount<T: Into<String>>(&mut self, mount: T) -> &mut Self {
        self.containerconfig.add_mount(mount);
        self
    }

    pub fn set_publish_all(&mut self, publish_all: bool) -> &mut Self {
        self.containerconfig.set_publish_all(publish_all);
        self
    }

    pub fn set_compose<T: Into<String>>(&mut self, compose: T) -> &mut Self {
        self.dockersource.set_compose(compose);
        self
    }
    
    pub fn set_image<T: Into<String>>(&mut self, image: T) -> &mut Self {
        self.dockersource.set_image(image);
        self
    }

    pub fn set_dockerfile<T: Into<String>>(&mut self, dockerfile: T) -> &mut Self {
        self.dockersource.set_dockerfile(dockerfile);
        self
    }

    pub fn add_build_arg<T: Into<String>, S: Into<String>>(&mut self, key: T, value: S) -> &mut Self {
        self.dockersource.add_build_arg(key, value);
        self
    }

    pub fn build(self) -> Container {
        Container 
        {
            id: None,
            ip: None,
            config: self.containerconfig.build(),
            source: self.dockersource.build(),
            is_running: false,
            publish_ports: Vec::new(),
            ssh: None
        }
    }
} 

// Public methods for docker container 
impl Container {
    pub fn new(config: ContainerConfig, source: DockerSource) -> Self {
        Container 
        { 
            id: None, 
            ip: None, 
            publish_ports: Vec::new(), 
            is_running: false, 
            ssh: None,
            config, 
            source 
        }
    }
}

impl Container {
    /// Execute command remotely in docker container
    ///
    /// # Examples
    /// 
    /// ```
    /// ```
    fn execute<T: Into<String>>(&self, arg: T) -> Child {
        let command = format!("docker exec {} {}", &self.config.name.as_ref().unwrap(), &arg.into());
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

    pub fn add_port_mapping(&mut self, ext: u16, int: u16) {
        self.publish_ports.push(PortMapping::new(ext, int));
    }

    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    pub fn get_id(&self) -> &String {
        self.id.as_ref().unwrap()
    }
}

impl Run<(), String> for Container {
    /// Run docker container using a dockerfile or image
    fn run(&mut self) -> Result<(), String> {
        // Create network
        let network = create_network(self.config.network.clone());
        match network {
            Ok(_) => info!("Successfully created network"),
            Err(value) => error!("{}", value),
        }

        // Build and start container
        if self.source.dockerfile.is_some() && self.source.image.is_some() {
            self.build_from_dockerfile();
        }
        else if self.source.image.is_some() {
            self.build_from_image();
        }

        // get and set ip
        let result = self.load_ip();
            if let Err(error) = result {
                panic!("Unable to get ip of container {}", error);
            }

        // get and set port mappings
        let _ = self.load_ports();

        // Install ssh server
        info!("Installing shh server on {}", self.get_id());
        // Reformat sh script for linux distro
        if cfg!(target_os = "windows") {
            spawn_command(&"dos2unix scripts/docker/docker_ssh_init.sh".to_string());
        }

        let _ = spawn_command(&format!("docker cp scripts/docker/docker_ssh_init.sh {}:/", self.id.as_ref().unwrap())).wait();
        let _ = status_command(&format!("docker exec {} sh ../docker_ssh_init.sh", self.id.as_ref().unwrap()));
        // Start ssh server
        let _ = spawn_command(&format!("docker exec -d {} /usr/sbin/sshd -D", self.id.as_ref().unwrap())).wait();

        // Sleep to wait for ssh server to properly start
        sleep(Duration::from_secs(1));

        // Set ssh client
        let _ = self.load_ssh();

        Ok(())     
    }
}

impl Container {
    fn build_from_dockerfile(&mut self) {}
    fn build_from_image(&mut self) {
        // Create image
        let id = output_command(format!("docker run {} -it {}", self.config.parse(), self.source.image.as_ref().unwrap()));
        self.set_id(id.trim().to_string());
    }

    pub fn load_ip(&mut self) -> Result<(), String> {
        let ip = output_command(format!("docker inspect -f {{{{range.NetworkSettings.Networks}}}}{{{{.IPAddress}}}}{{{{end}}}} {}", self.id.as_ref().unwrap()));
        let ip = ip.replace("\n", "").trim().to_string();
        let ip = Ipv4Addr::from_str(ip.as_str());
        if let Ok(ip) = ip {
            self.ip = Some(IpAddr::V4(ip));
        }
        else if let Err(err) = ip {
            return Err(err.to_string());
        }
        
        Ok(()) 
    }

    pub fn load_ports(&mut self) -> Result<(), String> {
        let ports = output_command(format!("docker port {}", self.id.as_ref().unwrap()));
        for port in ports.split("\n").filter(|x| !x.is_empty()) {
            let (int, ext) = port.split_once("/").unwrap();
            let int = int.parse::<u16>().unwrap();
            let ext = ext.split(":").last().unwrap().parse::<u16>().unwrap();
            self.add_port_mapping(ext, int);
        }

        Ok(())
    }

    pub fn load_ssh(&mut self) -> Result<(), String> {
        let mut ssh = Ssh::new();
        ssh.connect(&"127.0.0.1".to_string(), self.get_ssh_port().unwrap(), &"root".to_string(), &"password".to_string());

        self.ssh = Some(ssh);
        Ok(())
    }
}
