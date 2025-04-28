use super::{super::command::command_func::spawn_command, container::Container, traits::EnvBuilder, ContainerType};


use log::error;

#[derive(Debug)]
pub struct MultiContainer {
    pub compose: Option<String>,
    pub containers: Vec<Container>,
}

impl Default for MultiContainer {
    fn default() -> Self {
        MultiContainer { compose: None, containers: Vec::<Container>::new() }
    }
}

impl MultiContainer {
    pub fn new<T: Into<String>>(compose: T, containers: Vec<Container>) -> Self {
        MultiContainer { compose: Some(compose.into()), containers }
    }

    pub fn set_compose<T: Into<String>>(&mut self, compose: T) -> &mut Self {
        self.compose = Some(compose.into());
        self
    }
}

impl EnvBuilder<(), String> for MultiContainer {
    fn validate(&self) -> bool {
        let mut valid = true;

        valid = self.compose.is_some();

        valid
    }

    fn build(&mut self) -> Result<(), String> {
        let valid = self.validate();
        if !valid {
            panic!("Not a valid multicontainer enviroment");
        }

        let result = spawn_command(format!("docker compose {} up -d --build", self.compose.as_ref().unwrap())).wait();

        if let Err(error) = result {
            error!("{}", error);
        }

        Ok(())
    }
}
