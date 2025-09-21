use clap::Parser;
use energy_store::{init_logger, Args};
use log::{info, warn};
use rdkafka::{consumer::{CommitMode, Consumer, StreamConsumer}, message::Headers, ClientConfig, Message};
use serde::{Deserialize, Serialize};
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() {
    info!("Starting");
    let args: Args = Args::parse();

    init_logger(args.level);
    
    kafka_consumer(args).await;
}

pub async fn kafka_consumer(args: Args) -> ! {
    let connection_string = format!("host=postgres port=5432 user=postgres password=postgres dbname=words");
    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    info!("Setting up consumer");

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", args.consumer)
        .set("group.id", "postgres_consumer")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .create()
        .expect("Unable to create consumer");
    
    let topics: Vec<&str> = args.topic
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
                let payload: Result<String, ()> = match m.payload_view::<str>() {
                    Some(Ok(s)) => {
                        info!("{}", s);
                        Ok(s.to_string())
                    },
                    None => Err(()),
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                        Err(())
                    }
                };
                let word = payload.unwrap();
                let result = client.execute("INSERT INTO words (word, count) VALUES ($1, 1) ON CONFLICT (word) DO UPDATE SET count = words.count + 1", &[&word]).await.unwrap();
                dbg!(result);
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}
