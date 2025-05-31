use core::f64;
use std::{env::args, thread, time::{Duration, SystemTime}};
use clap::Parser;
use energy_sensor::{init_logger, Arguments};
use log::{debug, info, LevelFilter};
use rand::{seq::IndexedRandom, Rng};
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

    let mut rng = rand::rng();
    let sign = vec![-1, 1];

    loop {
        for topic in topics.iter() {
            let mut value = Faker.fake::<f64>();

            let number = rng.random_range(0..100);

            if number < 10 {
                let energy_sign = sign.choose(&mut rng).unwrap().to_owned() as f64;
                value *= energy_sign * 10.0;
            }

            let package = json!({
                "id": id,
                "value": value
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
