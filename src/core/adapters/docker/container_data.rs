use std::{collections::HashMap, net::{IpAddr, Ipv4Addr}, process::{Child, Command, Stdio}, str::FromStr};

use log::debug;

use crate::shared::Amount;

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
    pub mount: Amount<String>,
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
            mount: Amount::None,
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
        Self::default()        
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
