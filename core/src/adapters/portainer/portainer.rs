use std::{collections::HashMap, thread, time::Duration};

use crate::{
    adapters::{ContainerBuilder, Local, agent::Agent, docker, traits::Executor},
    object::{Object, ObjectBuilder},
    types::Node,
};
use log::{error, info, warn};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

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

    #[serde(skip)]
    #[serde(default = "default_executor")]
    pub executor: Box<dyn Executor + Send + Sync>,
}

#[derive(Debug)]
struct PortainerResponse {
    message: String,
    details: String,
}

/// Temporary struct to easily deserialize the jwt token
#[derive(Debug, Serialize, Deserialize)]
struct JWT {
    jwt: String,
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

pub fn default_executor() -> Box<dyn Executor + Send + Sync> {
    Box::new(Local::new())
}

impl Default for Portainer {
    fn default() -> Self {
        Portainer {
            volume: default_volume(),
            host: default_host(),
            port: default_port(),
            username: default_username(),
            password: default_password(),
            jwt: String::new(),
            executor: default_executor(),
        }
    }
}

impl Portainer {
    pub fn build(&mut self) {
        let containers = docker::api::get_container_names(&*self.executor);
        if let Err(error) = containers {
            panic!("{}", error);
        }
        let containers = containers.unwrap();

        if containers.contains(&String::from("portainer")) {
            info!("Portainer container already exists");
        } else {
            info!("Setting up portainer");

            let volumes = docker::api::get_all_volumes(&*self.executor);
            if let Err(error) = volumes {
                panic!("{}", error);
            }
            let volumes = volumes.unwrap();

            if !volumes.contains(&String::from("portainer_data")) {
                let result = self
                    .executor
                    .exec("docker volume create portainer_data".to_string());
                if let Err(error) = result {
                    error!("{error}");
                }
            }

            let result = self.executor.exec(
                "docker run -d \
                    -p 8000:8000 \
                    -p 9443:9443 \
                    --name portainer \
                    --restart=always \
                    -v /var/run/docker.sock:/var/run/docker.sock \
                    -v portainer_data:/data portainer/portainer-ce:lts"
                    .to_string(),
            );
            if let Err(error) = result {
                error!("{error}");
            }

            // poll docker container
            let result = docker::api::poll_container("portainer", 30, &*self.executor);
            if let Err(error) = result {
                panic!("{}", error);
            }
            // Extra sleep for proper cert setup
            thread::sleep(Duration::from_secs(1));
        }

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(self.create_account());
        rt.block_on(self.authenticate_account());

        info!("Finished setting up portainer");
    }

    /// Create remote portainer agents as objects
    pub fn create_agents(&self, nodes: Vec<&Node>) -> Vec<Agent> {
        let mut agents = Vec::new();
        for node in nodes.iter() {
            agents.push(self.create_agent(node));
        }
        agents
    }

    /// Create a remote portainer agent as object
    pub fn create_agent(&self, node: &Node) -> Agent {
        let mut container = ContainerBuilder::default();
        container
            .publish_map(9001, 9001)
            .name("portainer_agent")
            .restart(docker::RestartTypes::Always)
            .volume("/var/run/docker.sock:/var/run/docker.sock")
            .volume("/var/lib/docker/volumes:/var/lib/docker/volumes")
            .volume("/:/host")
            .image("portainer/agent:2.27.6");

        let object = ObjectBuilder::default()
            .ignore_graph(true)
            .name(format!("portainer-agent-{}", node.host))
            .node(node.to_owned())
            .docker_container_builder(container)
            .build()
            .expect("Unable to build portainer agent object");

        Agent { object }
    }

    /// Add agent environment to portainer
    pub async fn add_agent(&self, node: &Node) {
        info!("Adding portainer agent");

        let mut params = HashMap::<String, String>::new();
        params.insert("Name".to_string(), format!("node-{}", node.host));
        params.insert("URL".to_string(), format!("tcp://{}:9001", node.host));
        params.insert("EndpointCreationType".to_string(), "2".to_string());
        params.insert("TLS".to_string(), true.to_string());
        params.insert("TLSSkipVerify".to_string(), true.to_string());
        params.insert("TLSSkipClientVerify".to_string(), true.to_string());

        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        let result = client
            .post(format!("https://{}:{}/api/endpoints", self.host, self.port))
            .bearer_auth(self.jwt.clone())
            .form(&params)
            .send()
            .await;

        if let Err(error) = result {
            error!("{error}");
        } else if let Ok(response) = result {
            let body = response.text().await.unwrap();
            info!("{:?}", body);
        }
    }

    /// Create portainer account
    pub async fn create_account(&self) {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        let response = client
            .post(format!(
                "https://{}:{}/api/users/admin/init",
                self.host, self.port
            ))
            .header("Content-Type", "application/json")
            .json(&json!({
                "Username": self.username,
                "Password": self.password
            }))
            .send()
            .await;

        if let Ok(response) = response {
            match response.status() {
                reqwest::StatusCode::OK => info!(
                    "User successfully created with credentials [Username: {}, Password: {}]",
                    self.username, self.password
                ),
                reqwest::StatusCode::CONFLICT => warn!("User already exists"),
                _ => error!("{:?}", response),
            }
        } else if let Err(response) = response {
            error!("{:?}", response);
        }
    }

    /// Login portainer account
    ///
    /// Required for interaction with the portainer api due to the need for the jwt key
    pub async fn authenticate_account(&mut self) {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        let response = client
            .post(format!("https://{}:{}/api/auth", self.host, self.port))
            .header("Content-Type", "application/json")
            .json(&json!({
                "Username": self.username,
                "Password": self.password
            }))
            .send()
            .await;

        if let Ok(response) = response {
            let body = response.text().await.unwrap();
            let jwt: JWT = serde_json::from_str(body.as_str()).unwrap();
            self.jwt = jwt.jwt;
            info!("User successfully authenticated");
        } else if let Err(response) = response {
            error!("{:?}", response);
        }
    }
}
