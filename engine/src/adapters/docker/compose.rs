use std::collections::HashMap;

use crate::adapters::{Executor, Local};
use derive_builder::Builder;
use log::{debug, error};

use super::{Container, ContainerBuilder, Run};

/// The compose type. Represents `Docker compose` type
#[derive(Debug)]
pub struct Compose {
    pub config: ComposeConfig,
    pub containers: Vec<Container>,
    pub executor: Box<dyn Executor + Send + Sync>,
}

impl Compose {
    /// Get all containers that spawned from compose file
    pub fn get_containers<T: Into<String>>(&self, name: T) -> Option<&Container> {
        let name = name.into();
        for container in self.containers.iter() {
            if let Some(container_name) = container.config.name.as_ref()
                && container_name.eq(&name)
            {
                return Some(container);
            }
        }

        None
    }
}

impl Run for Compose {
    type Output = ();
    type Error = String;

    fn run(&mut self) -> Result<(), String> {
        if let Some(ref compose) = self.config.compose {
            let mut interpolation = String::new();
            for (key, value) in self.config.interpolation_variables.iter() {
                interpolation = format!("{interpolation} {key}={value}");
            }
            let result = self.executor.exec(format!(
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
            container.executor = self.executor.clone_box();
            container.set_id(id.clone());

            let result = super::api::poll_container(id.clone(), 30, &*self.executor);
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
                self.config.compose.as_ref().unwrap()
            );
            container.load();
        }

        Ok(())
    }
}

impl Compose {
    pub fn load_ids(&self) -> Vec<String> {
        let mut id_vec = Vec::new();
        let result = self.executor.exec(format!(
            "docker compose -f {} ps -q",
            self.config.compose.as_ref().unwrap()
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
    pub compose: Option<String>,
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
            config.compose.is_some(),
            "Docker compose requires a compose file"
        );

        let containers = Vec::new();
        let executor = Box::new(Local::new());

        Ok(Compose {
            config,
            containers,
            executor,
        })
    }
}

impl Default for ComposeConfig {
    fn default() -> Self {
        ComposeConfig {
            interpolation_variables: HashMap::new(),
            compose: None,
        }
    }
}

impl Default for Compose {
    fn default() -> Self {
        Compose {
            config: ComposeConfig::default(),
            executor: Box::new(Local::new()),
            containers: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ComposeBuilder;
    use crate::adapters::{Executor, ExecutorError, Run};
    use std::sync::{Arc, Mutex};

    #[derive(Debug, Clone)]
    pub struct DummyExecutor {
        output: Arc<Mutex<String>>,
    }

    impl Default for DummyExecutor {
        fn default() -> Self {
            let arc = Arc::new(Mutex::new(String::new()));
            DummyExecutor { output: arc }
        }
    }

    impl DummyExecutor {
        #[allow(dead_code)]
        pub fn new(mutex: Arc<Mutex<String>>) -> Self {
            DummyExecutor { output: mutex }
        }

        #[allow(dead_code)]
        pub fn get_output(&self) -> String {
            self.output.lock().unwrap().clone()
        }
    }

    impl Executor for DummyExecutor {
        fn exec(&self, command: String) -> Result<String, ExecutorError> {
            let mut output = self.output.lock().unwrap();
            *output = command.clone();
            Ok(command)
        }

        fn clone_box(&self) -> Box<dyn Executor + Send + Sync> {
            panic!()
        }
    }

    ////////////////////////////////////////////////
    /// Tests
    ////////////////////////////////////////////////

    #[test]
    #[should_panic]
    fn docker_no_compose() {
        let dummy = DummyExecutor::default();

        let mut compose = ComposeBuilder::default()
            .build()
            .expect("Unable to build compose");

        compose.executor = dummy.to_box_executor();

        let _ = compose.run();
    }
}
