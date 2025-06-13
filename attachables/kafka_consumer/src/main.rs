use std::fs;
use std::path::Path;
use kafka_consumer::arguments::Arguments;
use log::{info, error, warn};
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::{ClientConfig, Message};
use kafka_consumer::logger::init_logger;

#[tokio::main]
async fn main() {
    info!("Starting");

    let config_path = Path::new("config.json");
    let config = fs::read_to_string(config_path).expect("Unable to read config file");
    let args: Arguments = serde_json::from_str(config.as_str()).expect("Unable to parse config to struct");

    init_logger(args.level);

    kafka_consumer(args).await;
}

pub async fn kafka_consumer(args: Arguments) {
    info!("Setting up consumer");

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", args.consumer)
        .set("group.id", args.group_id)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .create()
        .expect("Unable to create consumer");
    
    let topics: Vec<&str> = args.topics
        .iter()
        .map(|x| x.as_str())
        .collect();
    
    consumer
        .subscribe(&topics)
        .expect("Unable to subscribe to topics");

    let client = reqwest::Client::new();

    info!("Listening on topic");
    loop {
        match consumer.recv().await {
            Err(e) => warn!("Kafka error: {}", e),
            Ok(m) => {
                match m.payload_view::<str>() {
                    Some(Ok(s)) => {
                        let s = s.to_owned();

                        let result = client.post(&args.address)
                            .body(s)
                            .send()
                            .await;

                        if let Err(error) = result {
                            error!("{}", error);
                        }
                    },
                    None => {
                        warn!("No payload");
                    },
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                    }
                };
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}
