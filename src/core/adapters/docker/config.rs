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
        ContainerConfigBuilder
        {
            containerconfig: ContainerConfig
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
}

impl ContainerConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_name<T: Into<String>>(&mut self, name: T) -> &mut Self {
        self.containerconfig.name = Some(name.into());
        self
    }

    pub fn try_set_name<T: Into<String>>(&mut self, name: T) -> &mut Self {
        if self.containerconfig.name.is_none() {
            self.containerconfig.name = Some(name.into());
        }
        self
    }

    pub fn set_network<T: Into<String>>(&mut self, network: T) -> &mut Self {
        self.containerconfig.network = network.into();
        self
    }

    pub fn add_env_var<T: Into<String>, S: Into<String>>(&mut self, key: T, value: S) -> &mut Self {
        self.containerconfig.enviroment.insert(key.into(), value.into());
        self
    }

    pub fn add_mount<T: Into<String>>(&mut self, mount: T) -> &mut Self {
        self.containerconfig.mount.push(mount.into());
        self
    }

    pub fn add_publish(&mut self, port: u16) -> &mut Self {
        self.containerconfig.publish.push(port);
        self
    }

    pub fn add_publish_map(&mut self, left: u16, right: u16) -> &mut Self {
        self.containerconfig.publish_map.push((left, right));
        self
    }

    pub fn set_publish_all(&mut self, publish_all: bool) -> &mut Self {
        self.containerconfig.publish_all = publish_all;
        self
    }

    pub fn set_expose(&mut self, expose: bool) -> &mut Self {
        self.containerconfig.expose = expose;
        self
    }

    pub fn build(self) -> ContainerConfig {
        self.containerconfig
    }
}
