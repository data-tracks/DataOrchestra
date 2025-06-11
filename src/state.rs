use std::sync::RwLock;

use crate::{api::api::BroadcastMessage, core::{config::Config, generate::Generate, object::Object, process::Process, store::Store}, shared::Arguments};

pub struct State {
    pub messages: RwLock<Vec<BroadcastMessage>>,
    config: Config,
    args: Arguments
}

impl State {
    pub fn new(config: Config, args: Arguments) -> Self {
        State 
        { 
            messages: RwLock::new(Vec::new()),
            args,
            config
        }
    }

    pub fn write_message(&mut self, message: BroadcastMessage) {
        if let Ok(writer) = self.messages.write().as_mut() {
            writer.push(message);
        }
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    } 

    pub fn get_stores(&self) -> &Vec<Store> {
        &self.config.store
    }

    pub fn get_processes(&self) -> &Vec<Process> {
        &self.config.process
    }

    pub fn get_generates(&self) -> &Vec<Generate> {
        &self.config.generate
    }

    pub fn get_objects(&self) -> &Vec<Object> {
        &self.config.object
    }

    pub fn get_args(&self) -> &Arguments {
        &self.args
    }
}
