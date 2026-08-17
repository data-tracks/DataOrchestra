use log::info;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::sync::mpsc::Receiver;

#[derive(Debug, Deserialize, Serialize)]
pub enum MessageType {
    Master,
    State,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub message_type: MessageType,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Master {
    ip: String,
    port: u16,
}

#[derive(Debug)]
pub struct StateManager {
    rx: Receiver<Message>,
    master: Option<Master>,
    running: bool,
}

impl StateManager {
    pub fn new(rx: Receiver<Message>) -> Self {
        StateManager {
            rx,
            master: None,
            running: false,
        }
    }

    pub fn set_master(&mut self, master: Master) {
        self.master = Some(master);
    }

    pub async fn run(mut self) {
        let r = self.rx.recv().await;
        info!("HERE RECEIVE");
        if let Some(message) = r {
            let master = serde_json::from_str(&message.message).expect("Error while parsing");
            self.master = Some(master);
        }

        loop {
            tokio::time::sleep(Duration::from_secs(2)).await;
            if self.master.is_none() {
                continue;
            }

            info!(
                "{} {}",
                self.master.as_ref().unwrap().ip,
                self.master.as_ref().unwrap().port
            );
            /*
            let body = reqwest::get(format!("http://{}:{}", master.ip, master.port).as_str())
                .await
                .expect("Error while fetching master state")
                .text()
                .await
                .expect("Error while parsing body");
            let message = Message {
                message_type: MessageType::State,
                message: body,
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
