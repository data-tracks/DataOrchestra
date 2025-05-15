use std::net::IpAddr;

use log::{error, info};
use serde::{Deserialize, Serialize};
use crate::core::{adapters::{command::command_func::output_command, docker, Executor}, types::Node};

#[derive(Debug, Serialize, Deserialize)]
pub struct Portainer {
    /// Portainer volume
    #[serde(default = "default_volume")]
    pub volume: String,

    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_username")]
    pub username: String,
    #[serde(default = "default_password")]
    pub password: String,

    /// JWT Token for authentication with API
    #[serde(skip)]
    jwt: String,
}

/// Temporary struct to easily deserialize the jwt token
#[derive(Debug, Serialize, Deserialize)]
struct JWT {
    token: String
}

pub fn default_volume() -> String {
    "portainer_data".to_string()
}

pub fn default_host() -> String {
    "localhost".to_string()
}

pub fn default_port() -> u16 {
    9443
}

pub fn default_username() -> String {
    "admin".to_string()
}

pub fn default_password() -> String {
    "portaineradmin".to_string()
}


impl Portainer {
    pub fn build(&mut self) {
        let containers = docker::get_all_container_names();
        if containers.contains(&String::from("portainer")) {
            error!("Portainer container already exists");
        }
        else {
            info!("Setting up portainer");

            let volumes = docker::get_all_volumes();
            if !volumes.contains(&String::from("portainer_data")) {
                let result = output_command("docker volume create portainer_data");
            }

            let result = output_command
                (
                    "docker run -d \
                    -p 8000:8000 \
                    -p 9443:9443 \
                    --name portainer \
                    --restart=always \
                    -v /var/run/docker.sock:/var/run/docker.sock \
                    -v portainer_data:/data portainer/portainer-ce:lts"
                );
            
            self.create_account();
            self.authenticate_account();

            info!("Finished setting up portainer");
        }
    }

    pub fn create_agents(nodes: &Vec<&Node>) {
        for node in nodes {
            if let Some(ref ssh) = node.ssh {
                ssh.exec(
                    "docker run -d \
                    -p 9001:9001 \
                    --name portainer_agent \
                    --restart=always \
                    -v /var/run/docker.sock:/var/run/docker.sock \
                    -v /var/lib/docker/volumes:/var/lib/docker/volumes \
                    -v /:/host \
                    portainer/agent:2.27.6"
                );
            }
            else {
                error!("Unable to setup portainer on node {} due to no availible ssh client", node.ip);
            }
        }
    }

    pub fn create_account(&self) {
        output_command(
            format!(
                "http POST {}:{}/api/users/admin/init \
                Username=\"{}\" \
                Password=\"{}\"", 
                self.host,
                self.port,
                self.username, 
                self.password
            ));
    }

    pub fn authenticate_account(&mut self) {
        let result = output_command(
            format!(
                "http POST {}:{}/api/auth \
                Username=\"{}\" \
                Password=\"{}\"", 
                self.host,
                self.port,
                self.username, 
                self.password
            ));
        let jwt: JWT = serde_json::from_str(result.as_str()).unwrap();
        self.jwt = jwt.token;
    }

    pub fn create_enviroment(&self, name: String, ip: IpAddr, port: u16) {
        let result = output_command(
            format!(
                "http --form POST https://{}:{}/api/endpoints \
                \"Authorization: Bearer {}\" \
                Name=\"{name}\" \
                URL=\"tcp://{ip}:{port}\" \
                EndpointCreationType=1",
                self.host,
                self.port,
                self.jwt,
            ));
    }
}
