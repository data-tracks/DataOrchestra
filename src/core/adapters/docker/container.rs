use core::panic;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use log::{debug, error};
use crate::core::adapters::ssh::Ssh;
use crate::core::adapters::traits::Runner;
use crate::core::adapters::{Local, OsSystems};

use super::config::ContainerConfigBuilder;
use super::source::DockerSourceBuilder;
use super::{ContainerConfig, DockerSource, Mount, PortMapping};
use super::Run;

/// The docker `Container` type. Represents the general information tied to the creation of a
/// docker container.
#[derive(Debug)]
pub struct Container {
    /// Id of container
    pub id: Option<String>,
    /// Ip of container
    pub ip: Option<IpAddr>,
    // Os system of container
    pub os: Option<OsSystems>,
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
    /// Local or remote command runner 
    pub runner: Box<dyn Runner + Send>
}

#[derive(Debug)]
pub struct ContainerBuilder {
    containerconfig: ContainerConfigBuilder,
    dockersource: DockerSourceBuilder,
}

impl Default for Container {
    fn default() -> Self {
        Container 
        { 
            id: None, 
            ip: None, 
            os: None, 
            publish_ports: Vec::new(), 
            is_running: false, 
            ssh: None, 
            config: ContainerConfig::default(), 
            source: DockerSource::default(), 
            runner: Box::new(Local::new()) 
        } 
    }
}

impl Default for ContainerBuilder {
    fn default() -> Self {
        ContainerBuilder 
        { 
            containerconfig: ContainerConfigBuilder::default(), 
            dockersource: DockerSourceBuilder::default() 
        }
    }
}

// Builder value setters and value cascaders for config and source.
//TODO: Replace when Bon cascade nightly is available
impl ContainerBuilder {
    pub fn new() -> ContainerBuilder {
        ContainerBuilder 
        { 
            containerconfig: ContainerConfigBuilder::default(), 
            dockersource: DockerSourceBuilder::default() 
        }
    }
    pub fn config(mut self, config: ContainerConfigBuilder) -> Self {
        self.containerconfig = config;
        self
    }

    pub fn config_mut(&mut self, config: ContainerConfigBuilder) -> &mut Self {
        self.containerconfig = config;
        self
    }
    
    pub fn source(mut self, source: DockerSourceBuilder) -> Self {
        self.dockersource = source;
        self
    }
    
    pub fn source_mut(&mut self, source: DockerSourceBuilder) -> &mut Self {
        self.dockersource = source;
        self
    }

    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.containerconfig = self.containerconfig.name(name);
        self
    }

    pub fn name_mut<T: Into<String>>(&mut self, name: T) -> &mut Self {
        self.containerconfig.name_mut(name);
        self
    }

    pub fn try_name<T: Into<String>>(mut self, name: T) -> Self {
        self.containerconfig = self.containerconfig.try_name(name);
        self
    }

    pub fn try_name_mut<T: Into<String>>(&mut self, name: T) -> &mut Self {
        self.containerconfig.try_name_mut(name);
        self
    }

    pub fn network<T: Into<String>>(mut self, network: T) -> Self {
        self.containerconfig = self.containerconfig.network(network);
        self
    }

    pub fn set_network_mut<T: Into<String>>(&mut self, network: T) -> &mut Self {
        self.containerconfig.network_mut(network);
        self
    }

    pub fn env_var<T: Into<String>, S: Into<String>>(mut self, key: T, value: S) -> Self {
        self.containerconfig = self.containerconfig.env_var(key, value);
        self
    }

    pub fn env_var_mut<T: Into<String>, S: Into<String>>(&mut self, key: T, value: S) -> &mut Self {
        self.containerconfig.env_var_mut(key, value);
        self
    }

    pub fn volume<T: Into<String>>(mut self, mount: T) -> Self {
        self.containerconfig = self.containerconfig.volume(mount);
        self
    }

    pub fn volume_mut<T: Into<String>>(&mut self, mount: T) -> &mut Self {
        self.containerconfig.volume_mut(mount);
        self
    }

    pub fn publish(mut self, port: u16) -> Self {
        self.containerconfig = self.containerconfig.publish(port);
        self
    }

    pub fn publish_mut(&mut self, port: u16) -> &mut Self {
        self.containerconfig.publish_mut(port);
        self
    }

    pub fn publish_map(mut self, left: u16, right: u16) -> Self {
        self.containerconfig = self.containerconfig.publish_map(left, right);
        self
    }

    pub fn publish_map_mut(&mut self, left: u16, right: u16) -> &mut Self {
        self.containerconfig.publish_map_mut(left, right);
        self
    }

    pub fn publish_all(mut self, publish_all: bool) -> Self {
        self.containerconfig = self.containerconfig.publish_all(publish_all);
        self
    }

    pub fn publish_all_mut(&mut self, publish_all: bool) -> &mut Self {
        self.containerconfig.publish_all_mut(publish_all);
        self
    }

    pub fn compose<T: Into<String>>(mut self, compose: T) -> Self {
        self.dockersource = self.dockersource.compose(compose);
        self
    }

    pub fn compose_mut<T: Into<String>>(&mut self, compose: T) -> &mut Self {
        self.dockersource.compose_mut(compose);
        self
    }

    pub fn image<T: Into<String>>(mut self, image: T) -> Self {
        self.dockersource = self.dockersource.image(image);
        self
    }
    
    pub fn image_mut<T: Into<String>>(&mut self, image: T) -> &mut Self {
        self.dockersource.image_mut(image);
        self
    }

    pub fn dockerfile<T: Into<String>>(mut self, dockerfile: T) -> Self {
        self.dockersource = self.dockersource.dockerfile(dockerfile);
        self
    }

    pub fn dockerfile_mut<T: Into<String>>(&mut self, dockerfile: T) -> &mut Self {
        self.dockersource.dockerfile_mut(dockerfile);
        self
    }

    pub fn build_arg<T: Into<String>, S: Into<String>>(mut self, key: T, value: S) -> Self {
        self.dockersource = self.dockersource.build_arg(key, value);
        self
    }

    pub fn build_arg_mut<T: Into<String>, S: Into<String>>(&mut self, key: T, value: S) -> &mut Self {
        self.dockersource.build_arg_mut(key, value);
        self
    }

    pub fn expose(mut self, expose: bool) -> Self {
        self.containerconfig = self.containerconfig.expose(expose);
        self
    }

    pub fn expose_mut(&mut self, expose: bool) -> &mut Self {
        self.containerconfig.expose_mut(expose);
        self
    }

    pub fn mount(mut self, mount: Mount) -> Self {
        self.containerconfig = self.containerconfig.mount(mount);
        self
    }

    pub fn mount_mut(&mut self, mount: Mount) -> &mut Self {
        self.containerconfig.mount_mut(mount);
        self
    }

    pub fn build(self) -> Container {
        Container 
        {
            id: None,
            ip: None,
            os: None,
            config: self.containerconfig.build(),
            source: self.dockersource.build(),
            is_running: false,
            publish_ports: Vec::new(),
            ssh: None,
            runner: Box::new(Local::new()) 
        }
    }
} 

impl Container {
    pub fn new(config: ContainerConfig, source: DockerSource) -> Self {
        Container 
        { 
            id: None, 
            ip: None, 
            os: None,
            publish_ports: Vec::new(), 
            is_running: false, 
            ssh: None,
            runner: Box::new(Local::new()),
            config, 
            source,
        }
    }

    pub fn os(&self) -> Option<&OsSystems> {
        self.os.as_ref()
    }
}

impl Container {
    /// Get host ssh port mapping from docker container
    pub fn get_ssh_port(&self) -> Option<u16> {
        self.get_external_port(22)     
    }

    /// Get the internal port mapped to the `host` port
    pub fn get_internal_port(&self, host: u16) -> Option<u16> {
        for portmap in &self.publish_ports {
            if portmap.get_host() == host {
                return Some(portmap.get_internal());
            }
        };

        return None;
    }
   
    /// Get the host port mapped to the `internal` port
    pub fn get_external_port(&self, internal: u16) -> Option<u16> {
        for portmap in &self.publish_ports {
            if portmap.get_internal() == internal {
                return Some(portmap.get_host());
            }
        };

        return None;
    }

    pub fn add_port_mapping(&mut self, ext: u16, int: u16) {
        self.publish_ports.push(PortMapping::new(ext, int));
    }

    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.config.name = Some(name);
        self
    }

    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    pub fn get_id(&self) -> &String {
        self.id.as_ref().unwrap()
    }
}

impl Run for Container {
    type Output = ();
    type Error = String;

    /// Run docker container using a dockerfile or image
    fn run(&mut self) -> Result<(), String> {
        // Build and start container
        if self.source.dockerfile.is_some() {
            let _ = self.build_from_dockerfile().map_err(|err| panic!("{}", err));
        }

        if self.source.image.is_some() {
            let _ = self.build_from_image().map_err(|err| panic!("{}", err));
        }

        if let Some(id) = self.id.as_ref() {
            let result = super::api::poll_container(id, 30, &self.runner);
            if let Err(error) = result {
                panic!("Polling docker container {} timeout after 30 seconds | {}", id, error);
            }
        }
        else {
            panic!("No id available for docker container");
        }

        let result = self.load_name();
        if let Err(error) = result {
            error!("Unable to get name of container | {}", error);
        }

        // get and set ip
        let result = self.load_ip();
        if let Err(error) = result {
            panic!("Unable to get ip of container | {}", error);
        }

        let _ = self.load_os();
        if let Err(error) = result {
            error!("Unable to get os from container {} | {}", self.config.name.as_ref().unwrap(), error);
        }

        // get and set port mappings
        let result = self.load_ports();
        if let Err(error) = result {
            error!("Unable to get ports from container {} | {}", self.config.name.as_ref().unwrap(), error);
        } 
        
        // Install ssh server
        let result = self.install_ssh();
        if let Err(error) = result {
            error!("Unable to install ssh server on container {} | {}", self.config.name.as_ref().unwrap(), error);
        }

        Ok(())     
    }
}

impl Container {
    fn build_from_dockerfile(&mut self) -> Result<(), String> {
        if let (Some(dockerfile), Some(image)) = (&self.source.dockerfile, &self.source.image) {
            let mut building_args = String::new();
            for (key, value) in self.source.build_args.iter() {
                building_args = format!(" {building_args} {key}={value}");
            }

            if !building_args.is_empty() {
                building_args = format!("--build-arg {building_args}");
            }
            
            self.runner.exec(format!("docker build -f {} {building_args} -t {} .", dockerfile, image))?;
        }
        else {
            return Err(format!("Please additionally provide an image name for your dockerfile under \"docker\": {{ \"image\": \"<image>\", \"dockerfile\": \"<dockerfile>\" }} "));
        }

        Ok(())
    }
    fn build_from_image(&mut self) -> Result<(), String>{
        // Create image
        let id = self.runner.exec(format!("docker run {} -it {}", self.config.parse(), self.source.image.as_ref().unwrap()))?;
        self.set_id(id.trim().to_string().replace("\n", ""));
        Ok(())
    }

    pub fn load_name(&mut self) -> Result<(), String> {
        let name = self.runner.exec(format!("docker inspect -f {{{{.Name}}}} {}", self.id.as_ref().unwrap()))?;
        let name = name
            .replace("/", "")
            .replace("\n", "");
        self.set_name(name);

        Ok(())
    }

    pub fn load_ip(&mut self) -> Result<(), String> {
        let ip = self.runner.exec(format!("docker inspect -f {{{{range.NetworkSettings.Networks}}}}{{{{.IPAddress}}}}{{{{end}}}} {}", self.id.as_ref().unwrap()))?;
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
        let ports = self.runner.exec(format!("docker port {}", self.id.as_ref().unwrap()))?;
        for port in ports.split("\n").filter(|x| !x.is_empty()) {
            let (int, ext) = port.split_once("/").unwrap();
            let int = int.parse::<u16>().unwrap();
            let ext = ext.split(":").last().unwrap().parse::<u16>().unwrap();
            self.add_port_mapping(ext, int);
        }

        Ok(())
    }

    pub fn load_ssh(&mut self, host: IpAddr) -> Result<(), String> {
        let mut ssh = Ssh::new();
        let _ = ssh.connect(&host.to_string(), self.get_ssh_port().unwrap(), &"root".to_string(), Some(&"password".to_string()));

        self.ssh = Some(ssh);
        Ok(())
    }

    /// Get os system of container
    pub fn load_os(&mut self) -> Result<(), String> {
        let result = self.runner.exec(format!("docker exec {} cat /etc/os-release", self.id.as_ref().unwrap()))?;
        let keys = result.split("\n");
        for entry in keys {
            let pair = entry.split_once("=").unzip();
            if let (Some(key), Some(value)) = pair {
                if key.eq("ID") {
                    let os = OsSystems::from_str(value);
                    if let Ok(os) = os {
                        self.os = Some(os)
                    }
                    else {
                        error!("Unable to get os for container {}", self.config.name.as_ref().unwrap());
                    }
                } 
            } 
            }

        Ok(()) 
    }

    pub fn install_ssh(&self) -> Result<(), String> {
        debug!("Installing shh server on {}", self.config.name.as_ref().unwrap());

        // Set correct install script for different distros
        if let Some(ref os) = &self.os {
            match os {
                OsSystems::Debian | OsSystems::Ubuntu => {
                    let script = String::from("apt_ssh_setup.sh");

                    // Reformat sh script for linux distro
                    if cfg!(target_os = "windows") {
                        self.runner.exec(format!("dos2unix scripts/docker/{}", script))?;
                    }

                    self.runner.exec(format!("docker cp scripts/docker/{} {}:/", script, self.id.as_ref().unwrap()))?;
                    self.runner.exec(format!("docker exec {} sh /{}", self.id.as_ref().unwrap(), script))?;
                    
                    // Start ssh server
                    self.runner.exec(format!("docker exec -d {} /usr/sbin/sshd -D", self.id.as_ref().unwrap()))?;

                },
                OsSystems::Alpine => {
                    let script = String::from("apk_ssh_setup.sh");

                    // Reformat sh script for alpine distro
                    if cfg!(target_os = "windows") {
                        self.runner.exec(format!("dos2unix scripts/docker/{}", script))?;
                    }
                    
                    self.runner.exec(format!("docker cp scripts/docker/{} {}:/", script, self.id.as_ref().unwrap()))?;
                    self.runner.exec(format!("docker exec -u root {} sh /{}", self.id.as_ref().unwrap(), script))?;
                    
                    // Start ssh server
                    self.runner.exec(format!("docker exec -d {} /usr/sbin/sshd -D", self.id.as_ref().unwrap()))?;
                }
                _ => ()
            };
        }
        
        // Sleep to wait for ssh server to properly start
        sleep(Duration::from_secs(2));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use crate::core::adapters::{Run, Runner};
    use super::ContainerBuilder;

    #[derive(Debug, Clone)]
    pub struct DummyRunner {
        output: Arc<Mutex<String>>,
    }

    impl Default for DummyRunner {
        fn default() -> Self {
            let arc = Arc::new(Mutex::new(String::new()));
            DummyRunner {output: arc}
        }
    }

    impl DummyRunner {
        #[allow(dead_code)]
        pub fn new(mutex: Arc<Mutex<String>>) -> Self {
            DummyRunner {
                output: mutex,
            }
        }

        #[allow(dead_code)]
        pub fn get_output(&self) -> String {
            self.output.lock().unwrap().clone()
        }
    }

    impl Runner for DummyRunner {
        fn exec(&self, command: String) -> Result<String, String> {
            let mut output = self.output.lock().unwrap();
            *output = command.clone();
            Ok(command)
        }

        fn clone_box(&self) -> Box<dyn Runner + Send> {
            panic!()
        }
    }

    ////////////////////////////////////////////////
    /// Tests    
    ////////////////////////////////////////////////
    
    #[test]
    #[should_panic]
    fn docker_no_image() {
        let dummy = DummyRunner::default();

        let mut container = ContainerBuilder::default()
            .name("rust")
            .build();

        container.runner = dummy.to_box_runner();

        let _ = container.run();
    }

    #[test]
    #[should_panic]
    fn docker_dockerfile_no_image() {
        let dummy = DummyRunner::default();

        let mut container = ContainerBuilder::default()
            .dockerfile("dockerfile")
            .name("rust")
            .build();

        container.runner = dummy.to_box_runner();

        let _ = container.run();
    } 
}
