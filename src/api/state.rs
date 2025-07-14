use std::sync::{Arc, Mutex, RwLock};

use crate::{core::{config::Config, generate::Generate, object::Object, process::Process, store::Store}, shared::Arguments};
use serde::{Deserialize, Serialize};

pub type State = Arc<Mutex<APIState>>;

/// The state object. Represents global state used by the orchestrator api to manange and hold
/// data.
pub struct APIState {
    /// Logging messages
    pub messages: RwLock<Vec<BroadcastMessage>>,
    /// Orchestra config
    config: Option<Config>,
    /// CLI arguments
    args: Arguments
}

impl Default for APIState {
    fn default() -> Self {
        APIState { messages: RwLock::new(Vec::new()), config: None, args: Arguments::default() }
    } 
}

#[derive(Deserialize, Serialize, Clone)]
pub struct BroadcastMessage {
    pub from: String,
    pub message: String
}

impl APIState {
    pub fn new(config: Config, args: Arguments) -> Self {
        APIState 
        { 
            messages: RwLock::new(Vec::new()),
            args,
            config: Some(config)
        }
    }

    pub fn write_message(&mut self, message: BroadcastMessage) {
        if let Ok(writer) = self.messages.write().as_mut() {
            writer.push(message);
        }
    }

    // Immutable getters for `Config`
    pub fn get_config(&self) -> Option<&Config> {
        self.config.as_ref()
    } 

    pub fn set_config(&mut self, config: Config) {
        self.config = Some(config);
    }

    pub fn get_stores(&self) -> Option<&Vec<Store>> {
        if let Some(config) = self.config.as_ref() {
            return Some(&config.store);
        }

        None
    }

    pub fn get_processes(&self) -> Option<&Vec<Process>> {
        if let Some(config) = self.config.as_ref() {
            return Some(&config.process);
        }

        None
    }

    pub fn get_generates(&self) -> Option<&Vec<Generate>> {
        if let Some(config) = self.config.as_ref() {
            return Some(&config.generate);
        }

        None
    }

    pub fn get_objects(&self) -> Option<&Vec<Object>> {
        if let Some(config) = self.config.as_ref() {
            return Some(&config.object);
        }

        None
    }

    pub fn get_args(&self) -> &Arguments {
        &self.args
    }
}
