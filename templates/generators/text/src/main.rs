use std::{thread, time::Duration};

use clap::Parser;
use fake::{locales::EN, Fake, Faker};
use log::{info, LevelFilter};
use rdkafka::{producer::{FutureProducer, FutureRecord}, ClientConfig};
use sensor::{arguments::{Args, StreamProcessor}, logger::init_logger};
use fake::faker::lorem::raw::Sentence;

#[tokio::main]
async fn main() {
    init_logger(LevelFilter::Debug);
    info!("Starting");
    let args: Args = Args::parse();

    if let Some(ref process_type) = args.stream_processor {
        match process_type {
            StreamProcessor::Kafka => 
            {
                kafka_producer(args).await;
            },
            _ => ()
        }
    }
    else {
        standard_producer(args);
    }
}

pub async fn kafka_producer(args: Args) {
    info!("Starting kafka producer");
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", args.address)
        .create()
        .expect("Unable to create kafka producer");

    if args.topic.is_none() {
        panic!("No topics provided for kafka");
    }

    let topics: Vec<String> = args.topic.unwrap(); 

    loop {
        for topic in topics.iter() {
            let val: String = Sentence(EN, 10..20).fake();
            let delivery_status = producer
                .send(
                    FutureRecord::to(topic)
                        .payload(&format!("{}", &val))
                        .key(""),
                    Duration::from_secs(60),
                )
                .await;
        }

        thread::sleep(Duration::from_secs(args.interval));
    }
}

pub fn standard_producer(args: Args) {
    loop {
        let val: String = Faker.fake(); 
        println!("{}", val);
    }
} 
