use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::thread::sleep;
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use log::{debug, error, info, warn};
use crate::docker::docker_struct::Container;
use crate::command::command_func::{output_command, spawn_command, status_command};
use crate::ssh::ssh_struct::ssh;

use super::docker_struct::{default_address, default_image, default_mount, default_name, default_network, default_options, default_target};

/// Factory for the creation of a docker container 
impl Container {
    /// Set name of docker container
    pub fn set_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Add network to docker container
    pub fn set_network(mut self, network: String) -> Self {
        self.network = network;
        self
    }

    /// Add image to docker container
    pub fn set_image(mut self, image: String) -> Self {
        self.image = Some(image);
        self
    }

    /// Add enviroment variables to docker container
    pub fn add_env_var(mut self, key: String, value: String) -> Self {
        if let Some(ref mut map) = self.options {
            map.insert(key, value);
        }
        else {
            let mut map = HashMap::<String,String>::new();
            map.insert(key, value);
            self.options = Some(map);
        }

        self
    }

    pub fn add_mount(mut self, mount: String, target: String) -> Self {
        let _ = format!("-v {}:{}", mount, target);
        self
    }


}

impl Container {
    /// Get [`Docker`] with filled with default values
    pub fn new() -> Self {
        Container {
            name: default_name(),
            address: default_address(),
            image: default_image(),
            network: default_network(),
            mount: default_mount(),
            target: default_target(),
            options: default_options()
        }
    }

    /// Initialise docker container based on specified data in `Docker` struct.
    pub fn init(&mut self) -> Result<(), String>{
        info!("Initializing docker container");
       
        if self.name == None {
            self.name = Some(self.image.as_ref().unwrap().clone());
        }
        
        // Create network
        let networks: String = output_command("docker network ls");
        if !networks.contains(&self.network) {
            info!("{}", format!("Creating network bridge {}", &self.network));
            let create_bridge = spawn_command(&format!("docker network create -d bridge {}", &self.network))
                .wait();

            if create_bridge.is_err() || (create_bridge.is_ok() && !&create_bridge.as_ref().unwrap().success()) {
                warn!("Unable to create bridge {} | Code : {}", &self.network, &create_bridge.unwrap().code().unwrap());
            }
            else {
                info!("Successfully created bridge [{}]", &self.network)
            }
        }

        // Create image
        let containers: String = output_command("docker ps");
        if containers.contains(&self.name.as_ref().unwrap().clone()){
            warn!("Re initiating container {}", &self.name.as_ref().unwrap());
            let _container_stop = status_command(&format!("docker stop {}", &self.name.as_ref().unwrap()));
            let _container_rm = status_command(&format!("docker rm {}", &self.name.as_ref().unwrap()));

        }
        let docker = spawn_command(&format!("docker run {}", self.get_options()))
            .wait();
        if docker.is_err() || (docker.is_ok() && !&docker.as_ref().unwrap().success()) {
            error!("Unable to create docker container [{}] | Code : {}", &self.name.as_ref().unwrap(), &docker.unwrap().code().unwrap());
            return Err("Unable to create docker container".to_string());
        }
        else {
            info!("Successfully created docker container [{}]", &self.name.as_ref().unwrap())
        }

        // get ip
        let ip_output = output_command(format!("docker inspect -f {{{{range.NetworkSettings.Networks}}}}{{{{.IPAddress}}}}{{{{end}}}} {}", &self.name.as_ref().unwrap()).as_str());

        let ip_output = ip_output.replace("\n", "");
        let ip_vec = ip_output.split(".").collect::<Vec<&str>>();
        let mut ip_vec_num = Vec::<u8>::new();
        for s in ip_vec {
            ip_vec_num.push(s.parse().unwrap());
        }
                
        self.address.ip = IpAddr::V4(Ipv4Addr::new(ip_vec_num[0], ip_vec_num[1], ip_vec_num[2], ip_vec_num[3]));

        // Install ssh server
        info!("Installing shh server on {}", &self.name.as_ref().unwrap());
        if cfg!(target_os = "windows") {
            // Reformat sh script for linux distro
            spawn_command(&"dos2unix src/docker/docker_ssh_init.sh".to_string());
        }

        let _ = spawn_command(&format!("docker cp src/docker/docker_ssh_init.sh {}:/", &self.name.as_ref().unwrap())).wait();
        let _ = status_command(&format!("docker exec {} sh ../docker_ssh_init.sh", &self.name.as_ref().unwrap()));

        // Start ssh server
        let _ = spawn_command(&format!("docker exec -d {} /usr/sbin/sshd -D", &self.name.as_ref().unwrap())).wait();
        
        // Sleep to wait for ssh server to properly start
        sleep(Duration::from_secs(1));


        Ok(())
    }

    /// Get docker container options as command input
    pub fn get_options(&self) -> String {
        let mut command: String = String::from("-d -q");

        command = format!("{command} --network={}", &self.network);

        command = format!("{command} --name={}", &self.name.as_ref().unwrap());

        // Publish ssh port
        command = format!("{command} -p {}:22", &self.address.port);
       
        // Publish postgres port
        if self.image.as_ref().unwrap() == "postgres" {
            command = format!("{command} -p 5432:5432");
        }

        if let Some(options) = &self.options {
            for (key, value) in options {
                command = format!("{command} -e {key}={value}")
            }
        }

        if let Some(source) = &self.mount {
            if let Some(target) = &self.target {
                command = format!("{command} --mount type=bind,source={source},target={target}");
            }
            else {
                warn!("Mount was specified but no target");
            }
        }

        command = format!("{command} -it {}", self.image.as_ref().unwrap());

        // Install ssh on docker 
        //
        // -p port:22
        //
        // apt-get update
        // apt-get install openssh-server
        // mkdir /var/run/sshd
        // echo "root:password" | chpasswd
        // echo "PermitRootLogin yes" >> /etc/ssh/sshd_config
        // /usr/sbin/sshd -D
        //
        // ssh root@localhost -p [port]
        // password: password

        command
    }

    pub fn get_ssh(&mut self) -> ssh {
        let mut ssh = ssh::new();
        ssh.connect(&"127.0.0.1".to_string(), self.address.port, &"root".to_string(), &"password".to_string());

        ssh
    }
    /// Execute command remotely in docker container
    ///
    /// # Examples
    /// 
    /// ```
    /// use DataOrchestra::command::Docker;
    /// let docker: Docker = Docker { image: "ubuntu" };
    /// docker.execute(&["pwd"])
    /// ```
    fn execute(&self, arg: &str) -> Child {
        debug!("{}", format!("Running command: docker exec -it {} {}", &self.name.as_ref().unwrap(), arg));
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .arg(format!("/C docker exec -it {} {}", &self.name.as_ref().unwrap(), arg))
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(format!("docker exec -it {} {}", &self.name.as_ref().unwrap(), arg))
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("failed to execute process")
        };

        output    
    }
}
