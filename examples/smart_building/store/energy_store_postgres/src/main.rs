use clap::Parser;
use energy_store::{init_logger, Args};
use log::{info, warn};
use rdkafka::{consumer::{CommitMode, Consumer, StreamConsumer}, message::Headers, ClientConfig, Message};
use serde::{Deserialize, Serialize};
use tokio_postgres::NoTls;

#[derive(Debug, Deserialize, Serialize)]
pub struct Value {
    pub value: f64
}

#[tokio::main]
async fn main() {
    info!("Starting");
    let args: Args = Args::parse();

    init_logger(args.level);
    
    kafka_consumer(args).await;
}

pub async fn kafka_consumer(args: Args) -> ! {
    let connection_string = format!("host=postgres port=5432 user=postgres password=postgres dbname=energy");
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
                let payload: Result<f64, ()> = match m.payload_view::<str>() {
                    Some(Ok(s)) => {
                        info!("{}", s);
                        let package: Value = serde_json::from_str(&s).expect("Unable to parse json to struct");
                        Ok(package.value)
                    },
                    None => Err(()),
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                        Err(())
                    }
                };
                let result = client.execute("INSERT INTO energy (value) VALUES ($1)", &[&payload.unwrap()]).await.unwrap();
                dbg!(result);
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}
