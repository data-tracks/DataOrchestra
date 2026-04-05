use derive_builder::Builder;
use log::info;
use crate::{config::Config, traits::Spawnable};

/// The pipeline type. Represents a pipeline creation of spawnable types ([Spawnable]) with intermediate hooks.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct Pipeline<'spawn, T, E> {
    /// Spawnable types
    #[builder(default, setter(each = "spawner"))]
    spawners: Vec<&'spawn mut (dyn Spawnable + Send)>,
    /// Runnable function for before the building stage of spawnables
    #[builder(default, setter(each = "before_build_hook"))]
    before_build_hooks: Vec<Box<dyn Fn() -> Result<T, E>>>,
    /// Runnable function for before the setup stage of spawnables
    #[builder(default, setter(each = "before_setup_hook"))]
    before_setup_hooks: Vec<Box<dyn Fn() -> Result<T, E>>>,
    /// Runnable function for before the deploy stage of spawnables
    #[builder(default, setter(each = "before_deploy_hook"))]
    before_deploy_hooks: Vec<Box<dyn Fn() -> Result<T, E>>>,
    #[builder(default, setter(each = "after_deploy_hook"))]
    after_deploy_hooks: Vec<Box<dyn Fn() -> Result<T, E>>>,
}

impl<'spawn, T, E> PipelineBuilder<'spawn, T, E> {
    /// Add config to the pipeline
    pub fn spawner_config(mut self, config: &'spawn mut Config) -> Self {
        for (spawner, _) in config.get_mut_spawners() {
            self = self.spawner(spawner);
        }
        self
    }
}

impl<'spawn, T, E> Pipeline<'spawn, T, E> {
    /// Run the pipeline
    pub fn run(mut self) -> Result<(), E> {
        info!("Build hook");
        for hook in self.before_build_hooks.iter() {
            hook()?;
        }

        info!("Build spawn");
        for spawner in self.spawners.iter_mut() {
            if spawner.state().is_not_running() {
                spawner.build();
            }
        }

        info!("Setup hook");
        for hook in self.before_setup_hooks.iter() {
            hook()?;
        }

        info!("Setup spawn");
        for spawner in self.spawners.iter_mut() {
            if spawner.state().is_not_running() {
                spawner.setup();
            }
        }

        info!("Deploy hook");
        for hook in self.before_deploy_hooks.iter() {
            hook()?;
        }

        info!("Deploy spawn");
        for spawner in self.spawners.iter_mut() {
            if spawner.state().is_not_running() {
                spawner.deploy();
            }
        }

        info!("Final hook");
        for hook in self.after_deploy_hooks.iter() {
            hook()?;
        }

        Ok(())
    }
}
