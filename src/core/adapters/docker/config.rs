use std::collections::HashMap;
use derive_builder::Builder;

use super::{Container, Mount, PortMapping, RestartTypes};

/// The container config object. Represents the possible configurations that can be made to a
/// docker container with the run command 
#[derive(Debug, Builder)]
#[builder(
    name = "ContainerBuilder",
    build_fn(name = "build_internal"),
    derive(Debug)
)]
pub struct ContainerConfig {
    /// Name of container
    #[builder(setter(strip_option, into), default)]
    pub name: Option<String>,
    /// Network name
    #[builder(default = "default_network()", setter(into))]
    pub network: String,
    /// Enviroment variables
    #[builder(setter(custom), default)]
    pub enviroment: HashMap<String, String>,
    /// Mounted volumes 
    #[builder(setter(each(name = "volume", into)), default)]
    pub volumes: Vec<String>,
    #[builder(setter(each(name = "mount", into)), default)]
    pub mounts: Vec<Mount>,
    /// Published ports
    #[builder(setter(custom), default)]
    pub publishes: Vec<u16>,
    /// Published mapped ports `external`:`internal`
    #[builder(setter(custom), default)]
    pub publish_map: Vec<PortMapping>,
    /// Publish all ports
    #[builder(default = "false")]
    pub publish_all: bool,
    /// Expose all ports
    #[builder(default = "false")]
    pub expose: bool,
    /// Restart policy
    #[builder(default)]
    pub restart: RestartTypes,
    /// Container image
    #[builder(setter(strip_option, into), default)]
    pub image: Option<String>,
    /// Container dockerfile
    #[builder(setter(strip_option, into), default)]
    pub dockerfile: Option<String>,
    /// Building args for dockerfile
    #[builder(setter(custom), default)]
    pub build_args: HashMap<String, String>,
    #[builder(default = "false")]
    pub ignore_ssh: bool
}

impl ContainerBuilder {
    pub fn try_name(&mut self, name: impl Into<String>) -> &mut Self {
        if self.name.is_none() {
            self.name(name);
        }
        self
    }

    pub fn environment(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        let hashmap = self.enviroment.get_or_insert_default();
        hashmap.insert(key.into(), value.into());
        self
    }

    pub fn publish_map(&mut self, external: u16, internal: u16) -> &mut Self {
        if self.ignore_ssh.is_none() || self.ignore_ssh.is_some_and(|ignore| !ignore) {
            let vec = self.publish_map.get_or_insert_default();
            vec.push(PortMapping::new(external, internal));
        }
        self
    }

    pub fn publish(&mut self, internal: u16) -> &mut Self {
        if self.ignore_ssh.is_none() || self.ignore_ssh.is_some_and(|ignore| !ignore) {
            let vec = self.publishes.get_or_insert_default();
            vec.push(internal);
        }
        self
    }

    pub fn build_arg(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        let hashmap = self.build_args.get_or_insert_default();
        hashmap.insert(key.into(), value.into());
        self
    }
}

pub fn default_network() -> String {
    "orchestra".to_string()
}

impl Default for ContainerConfig {
    fn default() -> Self {
        ContainerConfig
        {
            name: None,
            network: default_network(),
            enviroment: HashMap::new(),
            volumes: Vec::new(),
            mounts: Vec::new(),
            publishes: Vec::new(),
            publish_map: Vec::new(),
            publish_all: false,
            expose: false,
            restart: RestartTypes::default(),
            image: None, 
            dockerfile: None, 
            build_args: HashMap::new(),
            ignore_ssh: false
        }
    }
}

impl ContainerConfig {
    /// Parser to parse [`ContainerConfig`] to valid docker run command
    pub fn parse_options(&self) -> String {
        let mut command: String = String::from("-d -q");

        // Parse network variable
        command = format!("{command} --network={}", &self.network);

        // Parse name variable
        if let Some(ref name) = self.name {
            command = format!("{command} --name={name}");
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
            for port in self.publishes.iter() {
           command = format!("{command} -p {port}");
            }
            // Parse published mapped ports
            for map in self.publish_map.iter() {
                command = format!("{command} -p {}:{}", map.get_external(), map.get_internal());
            }
        }

        // Parse enviroment variables
        for (key, value) in &self.enviroment {
            command = format!("{command} -e {key}={value}")
        }

        // Parse volumes
        for value in &self.volumes {
            command = format!("{command} -v {value}");
        }

        // Parse mounts
        for mount in &self.mounts {
            command = format!("{command} type=bind,src={},", mount.src);
            if mount.read_only {
                command = format!("{command}ro,");
            }
            command = format!("{command}dst={}", mount.dst);
            if let Some(bind_propagation) = mount.bind_propagation.as_ref() {
                command = format!("{command},bind-propagation={bind_propagation}");
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

impl ContainerBuilder {
    pub fn build(&self) -> Result<Container, String> {
        let config = self.build_internal()
            .expect("Unable to build container config"); 

        Ok(Container 
            { 
                config,
                ..Container::default()
            }
        )
    }
}
