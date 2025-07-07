use core::f64;
use std::{thread, time::Duration};
use clap::Parser;
use energy_sensor::{init_logger, Arguments};
use log::{debug, info};
use rand::{seq::IndexedRandom, Rng};
use serde_json::json;
use fake::{Fake, Faker};
use chrono;

#[tokio::main]
async fn main() {
    info!("Starting");
    let args: Arguments = Arguments::parse();

    init_logger(args.level);

    data_producer(args).await;
}

pub async fn data_producer(args: Arguments) {
    let id = Faker.fake::<u16>();

    // Invalid data randomization 
    let mut rng = rand::rng();
    let sign = vec![-1, 1];

    let client = reqwest::Client::new();

    // Generate data
    loop {
        let mut value = Faker.fake::<f64>();

        let number = rng.random_range(0..100);

        if number < 10 {
            let energy_sign = sign.choose(&mut rng).unwrap().to_owned() as f64;
            value *= energy_sign * 10.0;
        }

        let package = json!({
            "id": id,
            "value": value,
            "timestamp": chrono::offset::Local::now()
        });

        dbg!("Sending package {}", &package);

        let result = client.post("http://kafka-producer:5000/kafkaproducer")
            .json(&package)
            .send()
            .await;

        debug!("{:?}", result);

        thread::sleep(Duration::from_secs(args.interval));
    }
}
