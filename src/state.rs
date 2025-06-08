use std::sync::RwLock;

use crate::{api::api::BroadcastMessage, core::{config::Config, generate::Generate, object::Object, process::Process, store::Store}};

pub struct State {
    pub messages: RwLock<Vec<String>>,
    config: Config
}

impl State {
    pub fn new(config: Config) -> Self {
        State 
        { 
            messages: RwLock::new(Vec::new()),
            config 
        }
    }

    pub fn write_message(&mut self, message: BroadcastMessage) {
        if let Ok(writer) = self.messages.write().as_mut() {
            let message = format!("[{}] {}", message.from, message.message);
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
}
