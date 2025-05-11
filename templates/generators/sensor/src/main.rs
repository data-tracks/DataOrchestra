use std::time::Duration;

use clap::Parser;
use fake::{Fake, Faker};
use rdkafka::{producer::{FutureProducer, FutureRecord}, ClientConfig};
use sensor::arguments::{Args, StreamProcessor};

fn main() {
    let args: Args = Args::parse();

    if let Some(ref process_type) = args.stream_processor {
        match process_type {
            StreamProcessor::Kafka => 
            {
                kafka_producer(args);
            },
            _ => ()
        }
    }
    else {
        standard_producer(args);
    }
}

pub async fn kafka_producer(args: Args) {
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
            let val: String = Faker.fake();
            let delivery_status = producer
                .send(
                    FutureRecord::to(topic)
                        .payload(&format!("{}", &val))
                        .key(""),
                    Duration::from_secs(0),
                )
                .await;
        }
    }
}

pub fn standard_producer(args: Args) {
    loop {
        let val: String = Faker.fake(); 
        println!("{}", val);
    }
} 
