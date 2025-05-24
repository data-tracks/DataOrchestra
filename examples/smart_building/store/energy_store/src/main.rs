use std::env;

use clap::Parser;
use energy_store::{init_logger, Args};
use log::{info, warn, LevelFilter};
use mongodb::{options::{ClientOptions, ResolverConfig}, Client};
use rdkafka::{consumer::{CommitMode, Consumer, StreamConsumer}, message::Headers, ClientConfig, Message};

#[tokio::main]
async fn main() {
    init_logger(LevelFilter::Debug);
    info!("Starting");
    let args: Args = Args::parse();
    
    kafka_consumer(args).await;
}

pub async fn kafka_consumer(args: Args) {
    // Load the MongoDB connection string from an environment variable:
    let client_uri = "mongodb://root:example@localhost:27017/".to_string();
    // A Client is needed to connect to MongoDB:

    // An extra line of code to work around a DNS issue on Windows:

    let options = ClientOptions::parse(&client_uri).await.unwrap();

    let client = Client::with_options(options).unwrap();

    // Print the databases in our MongoDB cluster:

    println!("Databases:");
    if let Ok(list) = client.list_database_names().await {
        println!("{:?}", list); 
    }

    info!("Setting up consumer");

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", args.consumer)
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
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                if let Some(headers) = m.headers() {
                    for header in headers.iter() {
                        info!("  Header {:#?}: {:?}", header.key, header.value);
                    }
                }
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}
