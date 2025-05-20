use std::time::Duration;

use clap::Parser;
use futures::TryStreamExt;
use kafka_processor::arguments::Args;
use kafka_processor::logger::init_logger;
use log::{info, LevelFilter, error};
use thiserror::Error;

use rdkafka::config::ClientConfig;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::message::OwnedMessage;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::Message;

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
    NoPayload
}

pub fn process_input<'a>(message: OwnedMessage) -> Result<Vec<String>, PayloadError> {
    match message.payload_view::<str>() {
        Some(Ok(payload)) => {
            let s = payload.split(" ");
            let mut data = Vec::<String>::new();
            for s_i in s {
                data.push(s_i.to_string());
            }
            return Ok(data);
        }
        Some(Err(_)) => Err(PayloadError::InvalidPayload),
        None => Err(PayloadError::NoPayload),
    }
}

pub async fn processor(args: Args) {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.server", args.consumer)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "5000")
        .create()
        .expect("Unable to create consumer");

    let input_topic: Vec<&str> = args.input_topic
        .iter()
        .map(|x| x.as_str())
        .collect();

    consumer
        .subscribe(&input_topic)
        .expect("Unable to subscribe to topic");

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.server", args.producer)
        .create()
        .expect("Unable to create producer");

    let stream_processor = consumer.stream().try_for_each(|borrowed_message| {
        let producer = producer.clone();
        let output_topic = args.output_topic.clone();

        async move {
            let owned_message = borrowed_message.detach();
            tokio::spawn(async move {
                // The body of this block will be executed on the main thread pool,
                // but we perform `expensive_computation` on a separate thread pool
                // for CPU-intensive tasks via `tokio::task::spawn_blocking`.
                let parse =
                    tokio::task::spawn_blocking(|| process_input(owned_message))
                        .await
                        .expect("failed to wait for expensive computation");

                if let Err(error) = parse {
                    error!("{}", error);
                }
                else if let Ok(parse) = parse {
                    for topic in output_topic.iter() {
                        for item in parse.iter() {
                            let produce_future = producer.send(
                                FutureRecord::to(topic)
                                    .key("some key")
                                    .payload(&item),
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
