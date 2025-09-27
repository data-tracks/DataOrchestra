use tokio::sync::RwLock;

use crate::{
    core::{
        adapters::Portainer, config::Config, generate::Generate, object::Object, process::Process,
        store::Store,
    },
    shared::Arguments,
};
use serde::{Deserialize, Serialize};

/// The state object. Represents global state used by the orchestrator api to manange and hold
/// data.
#[derive(Debug, Default)]
pub struct State {
    /// Logging messages
    pub messages: RwLock<Vec<BroadcastMessage>>,
    /// Orchestra config
    pub config: RwLock<Config>,
    pub portainer: Option<Portainer>,
    /// CLI arguments
    pub args: Arguments,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BroadcastMessage {
    pub from: String,
    pub message: String,
}

impl State {
    pub fn new(config: Config, args: Arguments) -> Self {
        State {
            messages: RwLock::new(Vec::new()),
            args,
            portainer: None,
            config: RwLock::new(config),
        }
    }

    pub async fn write_message(&mut self, message: BroadcastMessage) {
        let mut messages = self.messages.write().await;
        messages.push(message);
    }
}
