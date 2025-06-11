use crate::shared::traits::Spawner;

use super::Generate;

impl Spawner for Generate {
    fn build(&mut self) {
        self.object.build(); 
    }

    fn setup(&mut self) {
        self.object.setup();
    }
    
    fn deploy(&mut self) {
        self.object.deploy(); 
    }
}
