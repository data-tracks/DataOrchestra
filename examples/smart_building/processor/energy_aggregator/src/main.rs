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

use serde_json::{self, json};

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

#[derive(Debug, Deserialize, Serialize)]
pub struct Energy {
    pub id: u16,
    pub value: f64
}

pub fn process_input<'a>(messages: Vec<OwnedMessage>) -> Result<String, PayloadError> {
    let mut energy_sum = 0.0;

    for message in messages.iter() {
        match message.payload_view::<str>() {
            Some(Ok(payload)) => {
                let package: Energy = serde_json::from_str(payload).expect("Unable to parse to json"); 
                debug!("Package received {:?}", &package);
                energy_sum += package.value;
            }
            Some(Err(_)) => { return Err(PayloadError::InvalidPayload); },
            None => { return Err(PayloadError::NoPayload); },
        }
    } 

    Ok(json!({ "value": energy_sum / (messages.len() as f64) }).to_string())
}

pub async fn processor(args: Args) {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", args.consumer)
        .set("group.id", "energy_aggregate")
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

    let chunk_stream = consumer.stream().try_ready_chunks(10);
    let stream_processor = chunk_stream.try_for_each(|borrowed_messages| {
        let producer = producer.clone();
        let output_topic = args.producer_topic.clone();

        async move {
            let mut owned_messages= Vec::<OwnedMessage>::new();
            for borrowed_message in borrowed_messages {
                owned_messages.push(borrowed_message.detach());
            }
            tokio::spawn(async move {
                let payload =
                    tokio::task::spawn_blocking(|| process_input(owned_messages))
                        .await
                        .expect("failed to wait for the processing of the input");

                if let Err(error) = payload {
                    error!("{}", error);
                }
                else if let Ok(payload) = payload {
                    for topic in output_topic.iter() {
                        let produce_future = producer.send(
                            FutureRecord::to(topic)
                                .key("energy")
                                .payload(&payload),
                            Duration::from_secs(0),
                        );
                        match produce_future.await {
                            Ok(delivery) => println!("Sent: {:?}", delivery),
                            Err((e, _)) => println!("Error: {:?}", e),
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
