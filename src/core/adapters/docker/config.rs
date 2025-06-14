use std::collections::HashMap;
use super::{Mount, PortMapping, RestartTypes};

/// The container config object. Represents the possible configurations that can be made to a
/// docker container with the run command 
#[derive(Debug)]
pub struct ContainerConfig {
    /// Name of container
    pub name: Option<String>,
    /// Network name
    pub network: String,
    /// Enviroment variables
    pub enviroment: HashMap<String, String>,
    /// Mounted volumes 
    pub volume: Vec<String>,
    pub mount: Vec<Mount>,
    /// Published ports
    pub publish: Vec<u16>,
    /// Published mapped ports `external`:`internal`
    pub publish_map: Vec<PortMapping>,
    /// Publish all ports
    pub publish_all: bool,
    /// Expose all ports
    pub expose: bool,
    /// Restart policy
    pub restart: RestartTypes,
    /// Docker compose file
    pub compose: Option<String>,
    /// Container image
    pub image: Option<String>,
    /// Container dockerfile
    pub dockerfile: Option<String>,
    /// Building args for dockerfile
    pub build_args: HashMap<String, String>,
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
            restart: RestartTypes::default(),
            compose: None, 
            image: None, 
            dockerfile: None, 
            build_args: HashMap::new()
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
