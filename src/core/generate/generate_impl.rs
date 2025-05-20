use log::info;
use crate::shared::traits::Spawner;

use super::Generate;

impl Spawner for Generate {
    fn build(&mut self) {
        info!("Building Generate");

        self.object.build(); 

        info!("Finished building Generate");
    }

    fn setup(&mut self) {
        info!("Setting up Generate");

        self.object.setup();

        info!("Finished setting up Generate");

    }
    
    fn deploy(&mut self) {
        info!("Deploying Generate");
        
        self.object.deploy(); 

        info!("Finished deploying Generate");
    }
}
