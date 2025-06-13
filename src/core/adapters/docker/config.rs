use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{BindPropagation, PortMapping, RestartTypes};

#[derive(Debug)]
pub struct ContainerConfig {
    pub name: Option<String>,
    pub network: String,
    pub enviroment: HashMap<String, String>,
    pub volume: Vec<String>,
    pub mount: Vec<Mount>,
    pub publish: Vec<u16>,
    pub publish_map: Vec<PortMapping>,
    pub publish_all: bool,
    pub expose: bool,
    pub restart: RestartTypes 
}



#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Mount {
    src: String,
    dst: String,
    #[serde(default)]
    read_only: bool,
    bind_propagation: Option<BindPropagation>
}

impl Mount {
    pub fn new<T: Into<String>, S: Into<String>>(src: T, dst: S, read_only: bool, bind_propagation: Option<BindPropagation>) -> Self {
        Mount { src: src.into(), dst: dst.into(), read_only, bind_propagation }
    }
}

impl Default for ContainerConfig {
    fn default() -> Self {
        ContainerConfig
        {
            name: None,
            network: String::from("orchestra"),
            enviroment: HashMap::new(),
            volume: Vec::new(),
            mount: Vec::new(),
            publish: Vec::new(),
            publish_map: Vec::new(),
            publish_all: false,
            expose: false,
            restart: RestartTypes::default() 
        }
    }
}

impl ContainerConfig {
    /// Parser to parse [`ContainerConfig`] to valid docker run command
    pub fn parse(&self) -> String {
        let mut command: String = String::from("-d -q");

        // Parse network variable
        command = format!("{command} --network={}", &self.network);

        // Parse name variable
        if let Some(ref name) = self.name {
            command = format!("{command} --name={}", name);
        }

        // Parse expose all
        if self.expose {
            command = format!("{command} --expose");
        }
   
        // Parse ports
        if self.publish_all {
            command = format!("{command} -P");
        }
        else {
            // Parse published ports
            for port in self.publish.iter() {
                command = format!("{command} -p {}", port);
            }
            // Parse published mapped ports
            for map in self.publish_map.iter() {
                command = format!("{command} -p {}:{}", map.get_host(), map.get_internal());
            }
        }

        // Parse enviroment variables
        for (key, value) in &self.enviroment {
            command = format!("{command} -e {key}={value}")
        }

        // Parse volumes
        for value in &self.volume {
            command = format!("{command} -v {}", value);
        }

        // Parse mounts
        for mount in &self.mount {
            command = format!("{command} type=bind,src={},", mount.src);
            if mount.read_only {
                command = format!("{command}ro,");
            }
            command = format!("{command}dst={}", mount.dst);
            if let Some(bind_propagation) = mount.bind_propagation.as_ref() {
                command = format!("{command},bind-propagation={}", bind_propagation);
            }
        }

        // Parse restart
        match self.restart {
            RestartTypes::No => command = format!("{command} --restart=no"),
            RestartTypes::Always => command = format!("{command} --restart=always"),
            RestartTypes::UnlessStopped => command = format!("{command} --restart=unless-stopped"),
            RestartTypes::OnFailure(max_retries) => command = format!("{command} --restart=on-failure:{max_retries}"),
        }

        command
    }
}

#[derive(Debug)]
pub struct ContainerConfigBuilder {
    ignore_ssh: bool,
    containerconfig: ContainerConfig,
}

impl Default for ContainerConfigBuilder {
    fn default() -> Self {
        Self 
        {
            ignore_ssh: false,
            containerconfig: ContainerConfig::default()
        } 
    }
}

impl ContainerConfigBuilder {
    pub fn get_name(&self) -> Option<&String> {
        self.containerconfig.name.as_ref()
    }
}

impl ContainerConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ignore_ssh(mut self, ignore_ssh: bool) -> Self {
        self.ignore_ssh = ignore_ssh;
        self
    }

    pub fn ignore_ssh_mut(&mut self, ignore_ssh: bool) -> &mut Self {
        self.ignore_ssh = ignore_ssh;
        self
    }

    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.containerconfig.name = Some(name.into());
        self
    }

    pub fn name_mut<T: Into<String>>(&mut self, name: T) -> &mut Self {
        self.containerconfig.name = Some(name.into());
        self
    }

    pub fn try_name<T: Into<String>>(mut self, name: T) -> Self {
        if self.containerconfig.name.is_none() {
            self.containerconfig.name = Some(name.into());
        }
        self
    }

    pub fn try_name_mut<T: Into<String>>(&mut self, name: T) -> &mut Self {
        if self.containerconfig.name.is_none() {
            self.containerconfig.name = Some(name.into());
        }
        self
    }

    pub fn network<T: Into<String>>(mut self, network: T) -> Self {
        self.containerconfig.network = network.into();
        self
    }

    pub fn network_mut<T: Into<String>>(&mut self, network: T) -> &mut Self {
        self.containerconfig.network = network.into();
        self
    }

    pub fn env_var<T: Into<String>, S: Into<String>>(mut self, key: T, value: S) -> Self {
        self.containerconfig.enviroment.insert(key.into(), value.into());
        self
    }

    pub fn env_var_mut<T: Into<String>, S: Into<String>>(&mut self, key: T, value: S) -> &mut Self {
        self.containerconfig.enviroment.insert(key.into(), value.into());
        self
    }

    pub fn volume<T: Into<String>>(mut self, mount: T) -> Self {
        self.containerconfig.volume.push(mount.into());
        self
    }

    pub fn volume_mut<T: Into<String>>(&mut self, mount: T) -> &mut Self {
        self.containerconfig.volume.push(mount.into());
        self
    }

    pub fn publish(mut self, port: u16) -> Self {
        if !(port == 22 && self.ignore_ssh) {
            self.containerconfig.publish.push(port);
        }
        self
    }

    pub fn publish_mut(&mut self, port: u16) -> &mut Self {
        if !(port == 22 && self.ignore_ssh) {
            self.containerconfig.publish.push(port);
        }
        self
    }

    pub fn publish_map(mut self, left: u16, right: u16) -> Self {
        if !(right == 22 && self.ignore_ssh) {
            self.containerconfig.publish_map.push(PortMapping::new(left, right));
        }
        self
    }

    pub fn publish_map_mut(&mut self, left: u16, right: u16) -> &mut Self {
        if !(right == 22 && self.ignore_ssh) {
            self.containerconfig.publish_map.push(PortMapping::new(left, right));
        }
        self
    }

    pub fn publish_all(mut self, publish_all: bool) -> Self {
        self.containerconfig.publish_all = publish_all;
        self
    }

    pub fn publish_all_mut(&mut self, publish_all: bool) -> &mut Self {
        self.containerconfig.publish_all = publish_all;
        self
    }

    pub fn expose(mut self, expose: bool) -> Self {
        self.containerconfig.expose = expose;
        self
    }

    pub fn expose_mut(&mut self, expose: bool) -> &mut Self {
        self.containerconfig.expose = expose;
        self
   }

    pub fn mount(mut self, mount: Mount) -> Self {
        self.containerconfig.mount.push(mount);
        self
    }

    pub fn mount_mut(&mut self, mount: Mount) -> &mut Self {
        self.containerconfig.mount.push(mount);
        self
    }

    pub fn restart(mut self, restart: RestartTypes) -> Self {
        self.containerconfig.restart = restart;
        self
    }

    pub fn restart_mut(&mut self, restart: RestartTypes) -> &mut Self {
        self.containerconfig.restart = restart;
        self
    }

    pub fn build(self) -> ContainerConfig {
        self.containerconfig
    }

}
