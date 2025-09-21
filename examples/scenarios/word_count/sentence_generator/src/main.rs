use std::{thread, time::Duration};

use clap::Parser;
use fake::{locales::EN, Fake, Faker};
use log::{info, LevelFilter, debug};
use rdkafka::{producer::{FutureProducer, FutureRecord}, ClientConfig};
use sensor::logger::init_logger;
use fake::faker::lorem::raw::Sentence;
use sensor::arguments::Arguments;

#[tokio::main]
async fn main() {
    init_logger(LevelFilter::Debug);
    info!("Starting");
    let args: Arguments = Arguments::parse();

    kafka_producer(args).await;
}

pub async fn kafka_producer(args: Arguments) {
    info!("Starting kafka producer");
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", args.producer)
        .create()
        .expect("Unable to create kafka producer");

    if args.topic.is_none() {
        panic!("No topics provided for kafka");
    }

    let topics: Vec<String> = args.topic.unwrap();

    let client = reqwest::Client::new();

    loop {
        for topic in topics.iter() {
            let val: String = Sentence(EN, 10..20).fake();

            dbg!("Sending package {}", &val);

            let result = client.post("http://kafka-producer:5000/kafkaproducer")
                .body(val)
                .send()
                .await;

            debug!("{:?}", result);
        }

        thread::sleep(Duration::from_secs(args.interval));
    }
}
