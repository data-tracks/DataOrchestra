use std::{env::args, thread, time::{Duration, SystemTime}};
use clap::Parser;
use energy_sensor::{init_logger, Arguments};
use log::{debug, info, LevelFilter};
use rdkafka::{producer::{FutureProducer, FutureRecord}, ClientConfig};
use serde_json::json;
use fake::{Fake, Faker};

#[tokio::main]
async fn main() {
    init_logger(LevelFilter::Debug);
    info!("Starting");
    let args: Arguments = Arguments::parse();

    kafka_producer(args).await;
}

pub async fn kafka_producer(args: Arguments) {
    dbg!(&args);
    info!("Starting kafka producer");
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", args.address)
        .create()
        .expect("Unable to create kafka producer");

    if args.topic.is_none() {
        panic!("No topics provided for kafka");
    }

    let topics: Vec<String> = args.topic.unwrap(); 

    let id = Faker.fake::<u16>();

    let time = SystemTime::now();

    loop {
        for topic in topics.iter() {
            let package = json!({
                "id": id,
                "value": Faker.fake::<f64>()
            });
            debug!("Sending package {}", &package);

            let delivery_status = producer
                .send(
                    FutureRecord::to(topic)
                        .payload(&package.to_string())
                        .key(""),
                    Duration::from_secs(60),
                )
                .await;
        }

        thread::sleep(Duration::from_secs(args.interval));
    }
}
