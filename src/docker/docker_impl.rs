use std::fmt::format;
use std::net::{IpAddr, Ipv4Addr};
use std::thread::sleep;
use std::os::unix::thread;
use std::process::{Command, Child, Stdio};
use std::time::Duration;
use log::{debug, error, info, warn};
use crate::docker::docker_struct::Docker;
use crate::command::command_func::{output_command, spawn_command};
use crate::ssh::ssh_struct::ssh;

impl Docker {
    /// Initialise docker container based on specified data in `Docker` struct.
    pub fn init(&mut self) -> Result<(), String>{
        info!("Initializing docker container");
       
        if self.name == None {
            self.name = Some(self.image.clone());
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
                info!("Successfully created bridge \"{}\"", &self.network)
            }
        }

        // Create image
        let containers: String = output_command("docker ps");
        if containers.contains(&self.name.as_ref().unwrap().clone()){
            warn!("Re initiating container {}", &self.name.as_ref().unwrap());
            let _container_stop = spawn_command(&format!("docker stop {}", &self.name.as_ref().unwrap()))
                .wait();
            let _container_rm = spawn_command(&format!("docker rm {}", &self.name.as_ref().unwrap()))
                .wait();

        }
        let docker = spawn_command(&format!("docker run {}", self.get_options()))
            .wait();
        if docker.is_err() || (docker.is_ok() && !&docker.as_ref().unwrap().success()) {
            error!("Unable to create docker container \"{}\" | Code : {}", &self.name.as_ref().unwrap(), &docker.unwrap().code().unwrap());
            return Err("Unable to create docker container".to_string());
        }
        else {
            info!("Successfully created docker container \"{}\"", &self.name.as_ref().unwrap())
        }

        // get ip
        let ip_output = output_command(format!("docker inspect -f {{{{range.NetworkSettings.Networks}}}}{{{{.IPAddress}}}}{{{{end}}}} {}", &self.name.as_ref().unwrap()).as_str());

        info!("Docker container \"{}\" ip : {:?}", &self.name.as_ref().unwrap(), &ip_output);
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
        let _ = spawn_command(&format!("docker exec -it {} sh ../docker_ssh_init.sh", &self.name.as_ref().unwrap())).wait();
        
        // Start ssh server
        let _ = spawn_command(&format!("docker exec -d -it {} /usr/sbin/sshd -D", &self.name.as_ref().unwrap())).wait();
        // Sleep to wait for ssh server to properly start
        sleep(Duration::from_secs(1));
        Ok(())
    }

    /// Get docker container options as command input
    pub fn get_options(&self) -> String {
        let mut command: String = String::from("-d -q");

        command = format!("{command} --network={}", &self.network);

        command = format!("{command} --name={}", &self.name.as_ref().unwrap());

        command = format!("{command} -p {}:{}", &self.address.port, &self.address.internal_port);
        
        if &self.image == "postgres" {
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

        command = format!("{command} -it {}", &self.image);

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

    fn get_port(&self) {
        
    }

    fn get_ip(&self) {
        
    }

    fn get_host(&self) {
        
    }
}
