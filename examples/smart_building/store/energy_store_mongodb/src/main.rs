use std::time::Duration;

use clap::Parser;
use energy_store::{init_logger, Args};
use log::{info, warn, error};
use mongodb::{bson::{doc, to_document, Document}, options::ClientOptions, Client};
use rdkafka::{consumer::{CommitMode, Consumer, StreamConsumer}, message::Headers, ClientConfig, Message};
use serde::{Deserialize, Serialize};
use serde_json;
use chrono::{self, DateTime, Local};

#[derive(Debug, Deserialize, Serialize)]
pub struct Energy {
    pub id: u16,
    pub value: f64,
    pub timestamp: DateTime<Local>
}

#[tokio::main]
async fn main() {
    info!("Starting");
    let args: Args = Args::parse();

    init_logger(args.level);
    
    kafka_consumer(args).await;
}

pub async fn kafka_consumer(args: Args) -> ! {
    let client_uri = format!("mongodb://{}:{}@{}/?authSource=admin", args.user, args.password, args.mongo_address);
    let options = ClientOptions::parse(&client_uri).await.unwrap();
    let client = Client::with_options(options).unwrap();

    let db = client.database(&args.database);

    let collection = db.collection::<Document>(&args.collection);

    info!("Setting up consumer");

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", args.consumer)
        .set("group.id", "mongodb_consumer")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .create()
        .expect("Unable to create consumer");
    
    let topics: Vec<&str> = args.consumer_topic
        .iter()
        .map(|x| x.as_str())
        .collect();
    
    consumer
        .subscribe(&topics)
        .expect("Unable to subscribe to topics");

    info!("Listening on topic");
    loop {
        match consumer.recv().await {
            Err(e) => warn!("Kafka error: {}", e),
            Ok(m) => {
                let payload: Result<Document, ()> = match m.payload_view::<str>() {
                    Some(Ok(s)) => {
                        info!("{}", s);
                        dbg!(&s);
                        let energy_json: Energy = serde_json::from_str(s).expect("Unable to parse json to struct"); 
                        let energy_bson = to_document(&energy_json).expect("Unable to parse struct to bson");
                        Ok(energy_bson)
                    },
                    None => Err(()),
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                        Err(())
                    }
                };
                if let Ok(bson) = payload {
                    let _ = collection.insert_one(bson).await;
                }
                /*
                info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                if let Some(headers) = m.headers() {
                    for header in headers.iter() {
                        info!("  Header {:#?}: {:?}", header.key, header.value);
                    }
                }
                */
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}
