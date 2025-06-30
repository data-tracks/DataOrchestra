use std::sync::RwLock;

use data_orchestra::{core::{config::Config, generate::Generate, object::Object, process::Process, store::Store}, shared::Arguments};
use serde::{Deserialize, Serialize};


/// The state object. Represents global state used by the orchestrator api to manange and hold
/// data.
pub struct State {
    /// Logging messages
    pub messages: RwLock<Vec<BroadcastMessage>>,
    /// Orchestra config
    config: Config,
    /// CLI arguments
    args: Arguments
}

impl Default for State {
    fn default() -> Self {
        State { messages: RwLock::new(Vec::new()), config: Config::default(), args: Arguments::default() }
    } 
}

#[derive(Deserialize, Serialize, Clone)]
pub struct BroadcastMessage {
    pub from: String,
    pub message: String
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

    // Immutable getters for `Config`

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
