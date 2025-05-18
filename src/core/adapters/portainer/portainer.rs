use std::{net::IpAddr, thread, time::Duration};

use log::{error, info, warn};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::core::{adapters::{command::command_func::output_command, docker, traits::Runner}, types::Node};

/// Portainer struct. Holds general configuration of the local portainer container
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
    pub jwt: String,
}

/// Temporary struct to easily deserialize the jwt token
#[derive(Debug, Serialize, Deserialize)]
struct JWT {
    jwt: String
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

impl Default for Portainer {
    fn default() -> Self {
        Portainer 
        { 
            volume: default_volume(), 
            host: default_host(), 
            port: default_port(), 
            username: default_username(), 
            password: default_password(), 
            jwt: String::new() 
        }
    }
}

impl Portainer {
    pub fn build(&mut self) {
        let containers = docker::api::get_container_names(None);
        if let Err(error) = containers {
            panic!("{}", error);
        }
        let containers = containers.unwrap();

        if containers.contains(&String::from("portainer")) {
            error!("Portainer container already exists");
        }
        else {
            info!("Setting up portainer");

            let volumes = docker::api::get_all_volumes(None);
            if let Err(error) = volumes {
                panic!("{}", error);
            }
            let volumes = volumes.unwrap();

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

            // poll docker container
            let result = docker::api::poll_container("portainer", 30, None);
            if let Err(error) = result {
                panic!("{}", error);
            }
            // Extra sleep for proper cert setup
            thread::sleep(Duration::from_secs(1));

            let rt = tokio::runtime::Runtime::new().unwrap(); 
            let _ = rt.block_on(self.create_account());
            let _ = rt.block_on(self.authenticate_account());

            info!("Finished setting up portainer");
        }
    }

    /// Create a remote portainer agent
    pub fn create_agent(&self, node: &Node) {
        if let Some(ref ssh) = node.ssh {
            let _ = ssh.exec(
                "docker run -d \
                -p 9001:9001 \
                --name portainer_agent \
                --restart=always \
                -v /var/run/docker.sock:/var/run/docker.sock \
                -v /var/lib/docker/volumes:/var/lib/docker/volumes \
                -v /:/host \
                portainer/agent:2.27.6".to_string()
            );
        }
        else {
            error!("Unable to setup portainer on node {} due to no availible ssh client", node.host);
        }
    }

    /// Create portainer account
    pub async fn create_account(&self) {
        let client = Client::builder()
            .danger_accept_invalid_certs(true) 
            .build().unwrap();

        let response = client
            .post(format!("https://{}:{}/api/users/admin/init", self.host, self.port))
            .header("Content-Type", "application/json")
            .json(&json!({
                "Username": self.username,
                "Password": self.password
            }))
            .send().await;
        
        if let Ok(response) = response {
            match response.status() {
                reqwest::StatusCode::OK => info!("User successfully created with credentials [Username: {}, Password: {}]", self.username, self.password),
                reqwest::StatusCode::CONFLICT => warn!("User already exists"),
                _ => error!("{:?}", response)
            }
        }
        else if let Err(response) = response {
            error!("{:?}", response);
        }
    }

    /// Login portainer account
    ///
    /// Required for interaction with the portainer api due to the need for the jwt key
    pub async fn authenticate_account(&mut self) {
        let client = Client::builder()
            .danger_accept_invalid_certs(true) 
            .build().unwrap();

        let response = client
            .post(format!("https://{}:{}/api/auth", self.host, self.port))
            .header("Content-Type", "application/json")
            .json(&json!({
                "Username": self.username,
                "Password": self.password
            }))
            .send().await;
        
        if let Ok(response) = response {
            let body = response.text().await.unwrap();
            let jwt: JWT = serde_json::from_str(body.as_str()).unwrap();
            self.jwt = jwt.jwt;
            info!("User successfully authenticated");
        }
        else if let Err(response) = response {
            error!("{:?}", response);
        }

        
    }

    /// Create a portainer enviroment
    pub fn create_enviroment(&self, name: String, ip: IpAddr, port: u16) {
        let _result = output_command(
            format!(
                "http --verify=no --form POST https://{}:{}/api/endpoints \
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
