use std::collections::HashMap;

use log::{debug, error, warn};
use crate::core::adapters::{Local, Runner};

use super::container::ContainerBuilder;
use super::{Container, Run};

#[derive(Debug)]
pub struct ComposeGroup {
    pub interpolation_variables: HashMap<String, String>,
    pub compose: Option<String>,
    pub names: Vec<String>,
    pub containers: Vec<Container>,
    pub runner: Box<dyn Runner + Send>
}

impl ComposeGroup {
    pub fn get_container<T: Into<String>>(&self, name: T) -> Option<&Container> {
        let name = name.into();
        for container in self.containers.iter() {
            if let Some(container_name) = container.config.name.as_ref() {
                if container_name.eq(&name) {
                    return Some(container);
                } 
            }
        }
        
        None
    }
}

impl Run for ComposeGroup {
    type Output = ();
    type Error = String;

    fn run(&mut self) -> Result<(), String> {
        if let Some(ref compose) = self.compose {
            let mut interpolation = String::new();
            for (key, value) in self.interpolation_variables.iter() {
                interpolation = format!("{interpolation} {}={}", key, value);
            }
            let result = self.runner.exec(format!("{interpolation} docker compose -f {} up -d --build", compose));
            if let Err(error) = result {
                error!("{}", error);
            }
        }
        else {
            panic!("No compose to execute");
        }

        // Set id of containers.
        // As the containers here are non specific yet, we can arbitrarily set the id(s)
        for name in self.names.clone() {
            let mut container = ContainerBuilder::new().build();
            container.runner = self.runner.clone_box();
            let result = self.runner.exec(format!("docker ps -aqf \"name={}\"", name));
            if let Err(ref error) = result {
                error!("{}", error);
            }
            let id = result.unwrap().replace("\n", "");
            container.set_name(name.clone());
            container.set_id(id.replace("\n", ""));
            self.containers.push(container);

            let result = super::api::poll_container(id, 30, &self.runner);
            if let Err(error) = result {
                error!("{}", error);
            }
        }

        for container in self.containers.iter_mut() {
            debug!("Setting up container {} for compose {}", container.config.name.as_ref().unwrap(), self.compose.as_ref().unwrap());
            let result = container.load_ip();
            if let Err(error) = result {
                panic!("Unable to get ip of container {}", error);
            }
            
            let _result = container.load_ports();

            let _result = container.load_os();
            
            if container.get_ssh_port().is_some() { 
                let _result = container.install_ssh();
            }
            else {
                warn!("No ssh port exposed for {}. Additional functionality is lost. Consider adding the ssh port <external>:22 to the published ports", container.config.name.as_ref().unwrap());
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ComposeGroupBuilder {
    composegroup: ComposeGroup
}

impl ComposeGroupBuilder {
    pub fn new() -> Self {
        ComposeGroupBuilder 
        { 
            composegroup:  ComposeGroup 
            { 
                compose: None, 
                containers: Vec::new(),
                runner: Box::new(Local::new()),
                names: Vec::new(),
                interpolation_variables: HashMap::new()
            }
        }
    }

    pub fn set_compose<T: Into<String>>(&mut self, compose: T) -> &mut Self {
        self.composegroup.compose = Some(compose.into());
        self
    }

    pub fn add_container(&mut self, container: Container) -> &mut Self {
        self.composegroup.containers.push(container);
        self
    }

    pub fn add_name<T: Into<String>>(&mut self, name: T) -> &mut Self {
        self.composegroup.names.push(name.into());
        self
    }

    pub fn add_interpolation_variable<T: Into<String>, S: Into<String>>(&mut self, key: T, value: S) -> &mut Self {
        self.composegroup.interpolation_variables.insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> ComposeGroup {
        self.composegroup
    }
}
