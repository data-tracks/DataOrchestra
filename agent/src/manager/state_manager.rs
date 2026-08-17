use std::sync::mpsc;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use log::info;
use serde::Deserialize;

pub struct Message {

}

#[derive(Debug, Deserialize)]
pub struct Master {
    ip: String,
    port: u16,
}

#[derive(Debug)]
pub struct StateManager {
    master: Master,
    running: bool,
    receiver: Receiver<Message>
}

impl StateManager {
    pub fn new(master: Master) -> Self {

        let (tx, rx) = channel();

        thread::Builder::new().name("state-poll".to_string()).spawn(|| {
            StateManager::start(tx);
        }).expect("Error while creating polling thread");

        StateManager { master, running: true, receiver: rx }
    }

    pub fn start(tx: Sender<Message>) {
        loop {
            tokio::time::sleep(Duration::from_secs(2)).await;
            if self.master.is_none() {
                continue;
            }

            };

            let client = reqwest::Client::new();
            let res = client
                .post(format!("localhost:{}", self.port))
                .body(serde_json::to_string(&message).expect("Error"))
                .send()
                .await;

            info!("{}", res.is_err());
            */
        }
    }
}