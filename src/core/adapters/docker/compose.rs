use std::collections::HashMap;

use crate::core::adapters::{Local, Runner};
use derive_builder::Builder;
use log::{debug, error};

use super::{Container, ContainerBuilder, Run};

#[derive(Debug)]
pub struct Compose {
    pub config: ComposeConfig,
    pub containers: Vec<Container>,
    pub runner: Box<dyn Runner + Send + Sync>,
}

impl Compose {
    /// Get all containers that spawned from compose file
    pub fn get_containers<T: Into<String>>(&self, name: T) -> Option<&Container> {
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

impl Run for Compose {
    type Output = ();
    type Error = String;

    fn run(&mut self) -> Result<(), String> {
        if let Some(ref compose) = self.config.file {
            let mut interpolation = String::new();
            for (key, value) in self.config.interpolation_variables.iter() {
                interpolation = format!("{interpolation} {key}={value}");
            }
            let result = self.runner.exec(format!(
                "{interpolation} docker compose -f {compose} up -d --build"
            ));
            if let Err(error) = result {
                error!("{error}");
            }
        } else {
            panic!("No compose to execute");
        }

        let ids = self.load_ids();

        // Set id of containers.
        // As the containers here are non specific yet, we can arbitrarily set the id(s)
        for id in ids {
            let mut container = ContainerBuilder::default()
                .build()
                .expect("Unable to build container");
            container.runner = self.runner.clone_box();
            container.set_id(id.clone());

            let result = super::api::poll_container(id.clone(), 30, &*self.runner);
            if let Err(error) = result {
                panic!("Polling docker container {id} timeout after 30 seconds ({error})");
            }

            container.is_running = true;

            self.containers.push(container);
        }

        // Load data from running docker containers spawned by compose file
        for container in self.containers.iter_mut() {
            debug!(
                "Setting up container for compose {}",
                self.config.file.as_ref().unwrap()
            );
            container.load();
        }

        Ok(())
    }
}

impl Compose {
    pub fn load_ids(&self) -> Vec<String> {
        let mut id_vec = Vec::new();
        let result = self.runner.exec(format!(
            "docker compose -f {} ps -q",
            self.config.file.as_ref().unwrap()
        ));
        if let Ok(ids) = result {
            let ids = ids.split("\n").filter(|item| !item.is_empty());
            for id in ids {
                id_vec.push(id.trim().to_string());
            }
        } else if let Err(error) = result {
            error!("{error}");
        }

        id_vec
    }
}

/// Compose config builder sitting ontop of [`Compose`] object.
#[derive(Debug, Clone, Builder)]
#[builder(
    name = "ComposeBuilder",
    build_fn(name = "build_internal"),
    derive(Debug)
)]
pub struct ComposeConfig {
    #[builder(setter(custom))]
    pub interpolation_variables: HashMap<String, String>,
    #[builder(setter(into, strip_option), default)]
    pub file: Option<String>,
}

impl ComposeBuilder {
    pub fn interpolation_variable(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> &mut Self {
        let hashmap = self.interpolation_variables.get_or_insert_default();
        hashmap.insert(key.into(), value.into());
        self
    }

    pub fn build(&self) -> Result<Compose, String> {
        let config = self
            .build_internal()
            .expect("Unable to build compose config");

        assert!(
            config.file.is_some(),
            "Docker compose requires a compose file"
        );

        let containers = Vec::new();
        let runner = Box::new(Local::new());

        Ok(Compose {
            config,
            containers,
            runner,
        })
    }
}

impl Default for ComposeConfig {
    fn default() -> Self {
        ComposeConfig {
            interpolation_variables: HashMap::new(),
            file: None,
        }
    }
}

impl Default for Compose {
    fn default() -> Self {
        Compose {
            config: ComposeConfig::default(),
            runner: Box::new(Local::new()),
            containers: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ComposeBuilder;
    use crate::core::adapters::{Run, Runner, RunnerError};
    use std::sync::{Arc, Mutex};

    #[derive(Debug, Clone)]
    pub struct DummyRunner {
        output: Arc<Mutex<String>>,
    }

    impl Default for DummyRunner {
        fn default() -> Self {
            let arc = Arc::new(Mutex::new(String::new()));
            DummyRunner { output: arc }
        }
    }

    impl DummyRunner {
        #[allow(dead_code)]
        pub fn new(mutex: Arc<Mutex<String>>) -> Self {
            DummyRunner { output: mutex }
        }

        #[allow(dead_code)]
        pub fn get_output(&self) -> String {
            self.output.lock().unwrap().clone()
        }
    }

    impl Runner for DummyRunner {
        fn exec(&self, command: String) -> Result<String, RunnerError> {
            let mut output = self.output.lock().unwrap();
            *output = command.clone();
            Ok(command)
        }

        fn clone_box(&self) -> Box<dyn Runner + Send + Sync> {
            panic!()
        }
    }

    ////////////////////////////////////////////////
    /// Tests
    ////////////////////////////////////////////////

    #[test]
    #[should_panic]
    fn docker_no_compose() {
        let dummy = DummyRunner::default();

        let mut compose = ComposeBuilder::default()
            .build()
            .expect("Unable to build compose");

        compose.runner = dummy.to_box_runner();

        let _ = compose.run();
    }
}
