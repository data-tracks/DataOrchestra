use std::collections::HashMap;

#[derive(Debug)]
pub struct ContainerConfig {
    pub name: Option<String>,
    pub network: String,
    pub enviroment: HashMap<String, String>,
    pub mount: Vec<String>,
    pub publish: Vec<u16>,
    pub publish_map: Vec<(u16, u16)>,
    pub publish_all: bool,
    pub expose: bool 
}

impl Default for ContainerConfig {
    fn default() -> Self {
        ContainerConfig
        {
            name: None,
            network: String::from("orchestra"),
            enviroment: HashMap::new(),
            mount: Vec::new(),
            publish: Vec::new(),
            publish_map: Vec::new(),
            publish_all: false,
            expose: false
        }
    }
}

impl ContainerConfig {
    /// Parse container configuration to valid docker run command
    pub fn parse(&self) -> String {
        let mut command: String = String::from("-d -q");

        // Parse network variable
        command = format!("{command} --network={}", &self.network);

        // Parse name variable
        if let Some(ref name) = self.name {
            command = format!("{command} --name={}", name);
        }

        if self.expose {
            command = format!("{command} --expose");
        }
    
        if self.publish_all {
            command = format!("{command} -P");
        }
        else {
            // Publish ssh port
            command = format!("{command} -p 22");
            for port in self.publish.iter() {
                command = format!("{command} -p {}", port);
            }
            for (left, right) in self.publish_map.iter() {
                command = format!("{command} -p {left}:{right}");
            }
        }

        for (key, value) in &self.enviroment {
            command = format!("{command} -e {key}={value}")
        }

        // Parse mount 
        for value in &self.mount {
            command = format!("{command} -v {}", value);
        }

        command
    }
}

#[derive(Debug)]
pub struct ContainerConfigBuilder {
    containerconfig: ContainerConfig,
}

impl Default for ContainerConfigBuilder {
    fn default() -> Self {
        Self 
        {
            containerconfig: ContainerConfig::default()
        } 
    }
}

impl ContainerConfigBuilder {
    pub fn new() -> Self {
        Self::default()
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

    pub fn mount<T: Into<String>>(mut self, mount: T) -> Self {
        self.containerconfig.mount.push(mount.into());
        self
    }

    pub fn mount_mut<T: Into<String>>(&mut self, mount: T) -> &mut Self {
        self.containerconfig.mount.push(mount.into());
        self
    }

    pub fn publish(mut self, port: u16) -> Self {
        self.containerconfig.publish.push(port);
        self
    }

    pub fn publish_mut(&mut self, port: u16) -> &mut Self {
        self.containerconfig.publish.push(port);
        self
    }

    pub fn publish_map(mut self, left: u16, right: u16) -> Self {
        self.containerconfig.publish_map.push((left, right));
        self
    }

    pub fn publish_map_mut(&mut self, left: u16, right: u16) -> &mut Self {
        self.containerconfig.publish_map.push((left, right));
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

    pub fn build(self) -> ContainerConfig {
        self.containerconfig
    }
}

#[cfg(test)]
mod tests {
    use crate::core::adapters::ContainerBuilder;

    #[test]
    fn docker_network() {
        let mut container = ContainerBuilder::default()
            .name("rust")
            .image("rust")
            .network("docker_network")
            .build();
    }

    #[test]
    fn docker_name() {
        let mut container = ContainerBuilder::default()
            .name("rust")
            .image("rust")
            .network("docker_network")
            .build();
    }

    #[test]
    fn docker() {
        let mut container = ContainerBuilder::default()
            .name("rust")
            .image("rust")
            .network("docker_network")
            .build();
    }
}
