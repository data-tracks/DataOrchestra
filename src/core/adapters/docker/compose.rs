use std::{fs::{self, File}, path::Path};

use yaml_rust::YamlLoader;

use super::Container;

#[derive(Debug)]
pub struct ComposeGroup {
    compose: Option<String>,
    containers: Vec<Container>
}

impl ComposeGroup {
    fn _todo_(&mut self) {
        let compose = Path::new(self.compose.as_ref().unwrap());
        let yaml = fs::read_to_string(compose).expect("Unable to read compose");
        YamlLoader::load_from_str(yaml.as_str());
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


