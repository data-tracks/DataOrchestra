use clap::Parser;
use energy_store::{init_logger, Args};
use log::{info, warn, LevelFilter};
use mongodb::{bson::{doc, Document}, options::ClientOptions, Client};
use rdkafka::{consumer::{CommitMode, Consumer, StreamConsumer}, message::Headers, ClientConfig, Message};

#[tokio::main]
async fn main() {
    init_logger(LevelFilter::Debug);
    info!("Starting");
    let args: Args = Args::parse();
    
    kafka_consumer(args).await;
}

pub async fn kafka_consumer(args: Args) {
    let client_uri = format!("mongodb://{}:{}@{}/?authSource=admin", args.user, args.password, args.mongo_address);
    let options = ClientOptions::parse(&client_uri).await.unwrap();
    let client = Client::with_options(options).unwrap();

    let db = client.database("testdb");
    let collection = db.collection::<Document>("testcol");

    let filter = doc! { "name": "Alice" };
    if let Some(doc) = collection.find_one(filter).await.unwrap() {
        println!("✅ Found document: {:?}", doc);
    } else {
        println!("❌ No document found with that name.");
    }

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
