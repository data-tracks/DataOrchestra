use super::super::command::command_func::spawn_command;

use super::{container_data::Container, traits::EnvBuilder};

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

        let compose = self.compose.as_ref().unwrap();

        let result = spawn_command(format!("docker compose {} up -d --build", compose)).wait();

        if let Err(error) = result {
            error!("{}", error);
        }



        Ok(())
    }
}
