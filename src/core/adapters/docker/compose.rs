use std::path::Path;
use std::fs::{self};
use log::{debug, warn};
use yaml_rust::YamlLoader;
use crate::core::adapters::command::command_func::{output_command, spawn_command};
use super::container::ContainerBuilder;
use super::{Container, Run};

#[derive(Debug)]
pub struct ComposeGroup {
    pub compose: Option<String>,
    pub containers: Vec<Container>
}

impl ComposeGroup {
    pub fn get_names(&self) -> Vec<String>{
        let mut vec_names = Vec::<String>::new();
        let compose = Path::new(self.compose.as_ref().unwrap());
        let yaml = fs::read_to_string(compose).expect("Unable to read compose");
        let yaml = YamlLoader::load_from_str(yaml.as_str());
        if let Ok(yaml) = yaml {
            let doc = &yaml[0]["services"]; 
            for yaml in doc.as_hash() {
                for key in yaml.keys() {
                    let container = &yaml[key];
                    if !container["container_name"].is_badvalue() {
                        vec_names.push(container["container_name"].as_str().unwrap().to_string().replace("\n", ""));
                    }
                    else {
                        vec_names.push(key.as_str().unwrap().to_string().replace("\n", ""));
                    }
                }
            }
        }

        vec_names
    }
}

impl Run<(), String> for ComposeGroup {
    fn run(&mut self) -> Result<(), String> {
        if let Some(ref compose) = self.compose {
            let _ = spawn_command(format!("docker compose -f {} up -d --build", compose)).wait();
        }
        else {
            panic!("No compose to execute");
        }

        // Set id of containers.
        // As the containers here are non specific yet, we can arbitrarily set the id(s)
        let names: Vec<String> = self.get_names();
        for name in names {
            let mut container = ContainerBuilder::new().build();
            let id = output_command(format!("docker ps -aqf \"name={}\"", name));
            container.set_name(name);
            container.set_id(id.replace("\n", ""));
            self.containers.push(container);
        }

        for container in self.containers.iter_mut() {
            debug!("Setting up container {} for compose {}", container.config.name.as_ref().unwrap(), self.compose.as_ref().unwrap());
            let result = container.load_ip();
            if let Err(error) = result {
                panic!("Unable to get ip of container {}", error);
            }
            
            let _result = container.load_ports();

            let _result = container.load_os();
            
            let mut has_ssh= container.publish_ports
                .iter()
                .filter(|x| x.get_internal() == 22)
                .peekable();
            if has_ssh.peek().is_some() { 
                let _result = container.install_ssh();
                let _result = container.load_ssh();
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
                containers: Vec::new() 
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

    pub fn build(self) -> ComposeGroup {
        self.composegroup
    }
}


