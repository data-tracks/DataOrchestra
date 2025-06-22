use crate::{core::traits::Configurator, shared::traits::Spawner};

use super::Generate;

impl Spawner for Generate {
    fn build(&mut self) {
        /*
        if let Some(generate_type) = &self.generate_type {
            if self.config.is_none() {
                info!("No config was provided. Setting up default config");
                self.config = Some(.new());
            }
        }*/

        // Setup container based on specified config. Default setup if only process_type was provided,
        // otherwise custom
        if let Some(generate_config) = self.config.as_mut() {
            generate_config.configure(&mut self.object);
        }

        self.object.build(); 
    }

    fn setup(&mut self) {
        self.object.setup();
    }
    
    fn deploy(&mut self) {
        self.object.deploy(); 
    }
}
