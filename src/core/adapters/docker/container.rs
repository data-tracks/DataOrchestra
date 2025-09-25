use super::Run;
use super::{ContainerConfig, PortMapping};
use crate::core::adapters::docker::executor::DockerExecutor;
use crate::core::adapters::traits::Executor;
use crate::core::adapters::{Local, OsSystems};
use core::panic;
use log::{debug, error};
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;

/// The docker `Container` type. Represents the general information tied to the creation of a
/// docker container.
#[derive(Debug)]
pub struct Container {
    /// Container config
    pub config: ContainerConfig,
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
    /// Local or remote command executor
    pub executor: Box<dyn Executor + Send + Sync>,
    pub docker_executor: Box<dyn Executor + Send + Sync>,
}

impl Default for Container {
    fn default() -> Self {
        Container {
            id: None,
            ip: None,
            os: None,
            publish_ports: Vec::new(),
            is_running: false,
            config: ContainerConfig::default(),
            executor: Box::new(Local::new()),
            docker_executor: Box::new(DockerExecutor::default()),
        }
    }
}

impl Container {
    pub fn new(config: ContainerConfig) -> Self {
        Container {
            id: None,
            ip: None,
            os: None,
            publish_ports: Vec::new(),
            is_running: false,
            executor: Box::new(Local::new()),
            docker_executor: Box::new(DockerExecutor::default()),
            config,
        }
    }

    pub fn os(&self) -> Option<&OsSystems> {
        self.os.as_ref()
    }
}

impl Container {
    /// Get the internal port mapped to the `host` port
    pub fn get_internal_port(&self, host: u16) -> Option<u16> {
        for portmap in &self.publish_ports {
            if portmap.get_external() == host {
                return Some(portmap.get_internal());
            }
        }

        None
    }

    /// Get the host port mapped to the `internal` port
    pub fn get_external_port(&self, internal: u16) -> Option<u16> {
        for portmap in &self.publish_ports {
            if portmap.get_internal() == internal {
                return Some(portmap.get_external());
            }
        }

        None
    }

    /// Add port mapping of type `external`:`internal`
    pub fn add_port_mapping(&mut self, ext: u16, int: u16) {
        self.publish_ports.push(PortMapping::new(ext, int));
    }

    /// Set name of container
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.config.name = Some(name);
        self
    }

    /// Get name of container
    pub fn get_name(&self) -> Option<String> {
        self.config.name.clone()
    }

    /// Set id of container
    pub fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }

    /// Get id of container
    pub fn get_id(&self) -> &String {
        self.id.as_ref().unwrap()
    }
}

impl Run for Container {
    type Output = ();
    type Error = String;

    /// Run docker container using a dockerfile or image
    fn run(&mut self) -> Result<(), String> {
        self.create();

        if let Some(id) = self.id.as_ref() {
            debug!("Polling container {}", id);
            let result = super::api::poll_container(id, 30, &*self.executor);
            if let Err(error) = result {
                panic!("Polling docker container {id} timeout after 30 seconds ({error})");
            }
        } else {
            panic!("No id available for docker container");
        }

        self.is_running = true;

        self.load();

        Ok(())
    }
}

impl Container {
    pub fn create(&mut self) {
        // Build and start container
        if self.config.dockerfile.is_some() {
            let _ = self
                .build_from_dockerfile()
                .map_err(|err| panic!("{}", err));
        }

        if self.config.image.is_some() {
            let _ = self.build_from_image().map_err(|err| panic!("{}", err));
        }
    }

    pub fn load(&mut self) {
        let result = self.load_name();
        if let Err(error) = result {
            error!("Unable to get name of container ({error})");
        }

        // get and set ip
        let result = self.load_ip();
        if let Err(error) = result {
            panic!("Unable to get ip of container | {}", error);
        }

        let _ = self.load_os();
        if let Err(error) = result {
            error!(
                "Unable to get os from container {} | {}",
                self.config.name.as_ref().unwrap(),
                error
            );
        }

        // get and set port mappings
        let result = self.load_ports();
        if let Err(error) = result {
            error!(
                "Unable to get ports from container {} | {}",
                self.config.name.as_ref().unwrap(),
                error
            );
        }
    }

    /// Build dockerfile
    fn build_from_dockerfile(&mut self) -> Result<(), String> {
        if let (Some(dockerfile), Some(image)) = (&self.config.dockerfile, &self.config.image) {
            let mut building_args = String::new();
            for (key, value) in self.config.build_args.iter() {
                building_args = format!(" {building_args} {key}={value}");
            }

            if !building_args.is_empty() {
                building_args = format!("--build-arg {building_args}");
            }

            self.executor.exec(format!(
                "docker build -f {dockerfile} {building_args} -t {image} ."
            ))?;
        } else {
            return Err("Please additionally provide an image name for your dockerfile under \"docker\": {{ \"image\": \"<image>\", \"dockerfile\": \"<dockerfile>\" }} ".to_string());
        }

        Ok(())
    }

    /// Create container from image
    fn build_from_image(&mut self) -> Result<(), String> {
        // Create image
        let id = self
            .executor
            .exec(format!(
                "docker run {} -it {}",
                self.config.parse_options(),
                self.config.image.as_ref().unwrap()
            ))
            .map_err(|err| err.to_string())?;
        self.set_id(id.trim().to_string().replace("\n", ""));
        Ok(())
    }

    /// Load container name
    pub fn load_name(&mut self) -> Result<(), String> {
        let name = self
            .executor
            .exec(format!(
                "docker inspect -f {{{{.Name}}}} {}",
                self.id.as_ref().unwrap()
            ))
            .map_err(|err| err.to_string())?;
        let name = name.replace("/", "").replace("\n", "");
        self.set_name(name);

        Ok(())
    }

    /// Load container ip
    pub fn load_ip(&mut self) -> Result<(), String> {
        let ip = self.executor.exec(format!("docker inspect -f {{{{range.NetworkSettings.Networks}}}}{{{{.IPAddress}}}}{{{{end}}}} {}", self.id.as_ref().unwrap()))
            .map_err(|err| err.to_string())?;
        let ip = ip.replace("\n", "").trim().to_string();
        let ip_parse = Ipv4Addr::from_str(ip.as_str());
        if let Ok(ip) = ip_parse {
            self.ip = Some(IpAddr::V4(ip));
        } else if let Err(err) = ip_parse {
            return Err(format!("{err} ({ip})"));
        }

        Ok(())
    }

    /// Load all container port mappings
    pub fn load_ports(&mut self) -> Result<(), String> {
        let ports = self
            .executor
            .exec(format!("docker port {}", self.id.as_ref().unwrap()))
            .map_err(|err| err.to_string())?;
        for port in ports.split("\n").filter(|x| !x.is_empty()) {
            let (int, ext) = port.split_once("/").unwrap();
            let int = int.parse::<u16>().unwrap();
            let ext = ext.split(":").last().unwrap().parse::<u16>().unwrap();
            self.add_port_mapping(ext, int);
        }

        Ok(())
    }

    /// Get container os system
    pub fn load_os(&mut self) -> Result<(), String> {
        let result = self
            .executor
            .exec(format!(
                "docker exec {} cat /etc/os-release",
                self.id.as_ref().unwrap()
            ))
            .map_err(|err| err.to_string())?;
        let keys = result.split("\n");
        for entry in keys {
            let pair = entry.split_once("=").unzip();
            if let (Some(key), Some(value)) = pair {
                if key.eq("ID") {
                    let os = OsSystems::from_str(value);
                    if let Ok(os) = os {
                        self.os = Some(os)
                    } else {
                        error!(
                            "Unable to get os for container {}",
                            self.config.name.as_ref().unwrap()
                        );
                    }
                }
            }
        }

        Ok(())
    }

    /// Install ssh client on container
    pub fn install_ssh(&self) -> Result<(), String> {
        debug!(
            "Installing shh server on {}",
            self.config.name.as_ref().unwrap()
        );

        // Set correct install script for different distros
        if let Some(os) = &self.os {
            match os {
                OsSystems::Debian | OsSystems::Ubuntu => {
                    let script = String::from("apt_ssh_setup.sh");

                    // Reformat sh script for linux distro
                    if cfg!(target_os = "windows") {
                        self.executor
                            .exec(format!("dos2unix scripts/docker/{script}"))?;
                    }

                    self.executor.exec(format!(
                        "docker cp scripts/docker/{} {}:/",
                        script,
                        self.id.as_ref().unwrap()
                    ))?;
                    self.executor.exec(format!(
                        "docker exec {} sh /{}",
                        self.id.as_ref().unwrap(),
                        script
                    ))?;

                    // Start ssh server
                    self.executor.exec(format!(
                        "docker exec -d {} /usr/sbin/sshd -D",
                        self.id.as_ref().unwrap()
                    ))?;
                }
                OsSystems::Alpine => {
                    let script = String::from("apk_ssh_setup.sh");

                    // Reformat sh script for alpine distro
                    if cfg!(target_os = "windows") {
                        self.executor
                            .exec(format!("dos2unix scripts/docker/{script}"))?;
                    }

                    self.executor.exec(format!(
                        "docker cp scripts/docker/{} {}:/",
                        script,
                        self.id.as_ref().unwrap()
                    ))?;
                    self.executor.exec(format!(
                        "docker exec -u root {} sh /{}",
                        self.id.as_ref().unwrap(),
                        script
                    ))?;

                    // Start ssh server
                    self.executor.exec(format!(
                        "docker exec -d {} /usr/sbin/sshd -D",
                        self.id.as_ref().unwrap()
                    ))?;
                }
                _ => (),
            };
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::adapters::{ContainerBuilder, Executor, ExecutorError, Run};
    use std::sync::{Arc, Mutex};

    #[derive(Debug, Clone)]
    pub struct DummyExecutor {
        output: Arc<Mutex<String>>,
    }

    impl Default for DummyExecutor {
        fn default() -> Self {
            let arc = Arc::new(Mutex::new(String::new()));
            DummyExecutor { output: arc }
        }
    }

    impl DummyExecutor {
        #[allow(dead_code)]
        pub fn new(mutex: Arc<Mutex<String>>) -> Self {
            DummyExecutor { output: mutex }
        }

        #[allow(dead_code)]
        pub fn get_output(&self) -> String {
            self.output.lock().unwrap().clone()
        }
    }

    impl Executor for DummyExecutor {
        fn exec(&self, command: String) -> Result<String, ExecutorError> {
            let mut output = self.output.lock().unwrap();
            *output = command.clone();
            Ok(command)
        }

        fn clone_box(&self) -> Box<dyn Executor + Send + Sync> {
            panic!()
        }
    }

    ////////////////////////////////////////////////
    /// Tests
    ////////////////////////////////////////////////

    #[test]
    #[should_panic]
    fn docker_no_image() {
        let dummy = DummyExecutor::default();

        let mut container = ContainerBuilder::default()
            .name("rust")
            .build()
            .expect("Unable to build container");

        container.executor = dummy.to_box_executor();

        let _ = container.run();
    }

    #[test]
    #[should_panic]
    fn docker_dockerfile_no_image() {
        let dummy = DummyExecutor::default();

        let mut container = ContainerBuilder::default()
            .dockerfile("dockerfile")
            .name("rust")
            .build()
            .expect("Unable to build container");

        container.executor = dummy.to_box_executor();

        let _ = container.run();
    }
}
