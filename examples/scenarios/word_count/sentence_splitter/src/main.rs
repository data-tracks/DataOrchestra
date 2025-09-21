use std::time::Duration;

use clap::Parser;
use futures::TryStreamExt;
use kafka_processor::arguments::Args;
use kafka_processor::logger::init_logger;
use log::{debug, error, info, LevelFilter};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use rdkafka::config::ClientConfig;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::message::OwnedMessage;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::Message;

use serde_json;

#[tokio::main]
async fn main() {
    init_logger(LevelFilter::Debug);
    info!("Starting");
    let args: Args = Args::parse();

    processor(args).await;
}

#[derive(Debug, Error)]
pub enum PayloadError {
    #[error("Unable to parse payload to str")]
    InvalidPayload,
    #[error("No payload available to parse")]
    NoPayload,
    #[error("Generator produced invalid value. Filtering out value {0}")]
    InvalidValue(f64)
}


pub fn process_input<'a>(message: OwnedMessage) -> Result<Vec<String>, PayloadError> {
    let chars = ['@', '.', '\"', '\'', ',', ';', ':', '?', '!', '-', '_', '{', '\\', '}', '(', ')', '[', ']'];
    match message.payload_view::<str>() {
        Some(Ok(mut payload)) => {
            let mut filtered: String = payload.to_string();
            for char in chars {
                filtered = filtered.replace(char, "");
            }
            let split = filtered.split_whitespace()
                .into_iter()
                .map(|x| x.to_string())
                .collect();
            debug!("Package received {:?}", &split);

            Ok(split)
        }
        Some(Err(_)) => Err(PayloadError::InvalidPayload),
        None => Err(PayloadError::NoPayload),
    }
}

pub async fn processor(args: Args) {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", args.consumer)
        .set("group.id", "energy_filter")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .create()
        .expect("Unable to create consumer");

    let input_topic: Vec<&str> = args.consumer_topic
        .iter()
        .map(|x| x.as_str())
        .collect();

    consumer
        .subscribe(&input_topic)
        .expect("Unable to subscribe to topic");

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", args.producer)
        .create()
        .expect("Unable to create producer");

    let stream_processor = consumer.stream().try_for_each(|borrowed_message| {
        let producer = producer.clone();
        let output_topic = args.producer_topic.clone();

        async move {
            let owned_message = borrowed_message.detach();
            tokio::spawn(async move {
                let payload =
                    tokio::task::spawn_blocking(|| process_input(owned_message))
                        .await
                        .expect("failed to wait for the processing of the input");

                if let Err(error) = payload {
                    error!("{}", error);
                }
                else if let Ok(words) = payload {
                    for topic in output_topic.iter() {
                        for word in words.iter() {
                            let produce_future = producer.send(
                                FutureRecord::to(topic)
                                    .key("word")
                                    .payload(word),
                                Duration::from_secs(0),
                            );
                            match produce_future.await {
                                Ok(delivery) => println!("Sent: {:?}", delivery),
                                Err((e, _)) => println!("Error: {:?}", e),
                            }
                        }
                    }
                }
            });
            Ok(())
        }
    });

    info!("Starting event loop");
    stream_processor.await.expect("stream processing failed");
    info!("Stream processing terminated");
}
