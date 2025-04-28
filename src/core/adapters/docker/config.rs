use std::collections::HashMap;

#[derive(Debug)]
pub struct ContainerConfig {
    pub name: Option<String>,
    pub network: String,
    pub enviroment: HashMap<String, String>,
    pub mount: Vec<String>,
    pub publish_all: bool
}

impl ContainerConfig {
    pub fn parse(&self) -> String {
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

pub struct ContainerConfigBuilder {
    containerconfig: ContainerConfig,
}

impl ContainerConfigBuilder {
    pub fn new() -> Self {
        ContainerConfigBuilder 
        { 
            containerconfig: ContainerConfig 
            { 
                name: None, 
                network: String::from("orchestra"), 
                enviroment: HashMap::new(), 
                mount: Vec::new(), 
                publish_all: false 
            }
        }
    }

    pub fn set_name<T: Into<String>>(&mut self, name: T) -> &mut Self {
        self.containerconfig.name = Some(name.into());
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

    pub fn set_publish_all(&mut self, publish_all: bool) -> &mut Self {
        self.containerconfig.publish_all = publish_all;
        self
    }

    pub fn build(self) -> ContainerConfig {
        self.containerconfig
    }
}
