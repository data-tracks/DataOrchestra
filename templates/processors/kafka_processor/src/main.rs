use clap::Parser;
use kafka_processor::arguments::Args;
use kafka_processor::logger::init_logger;
use log::{info, LevelFilter};

use rdkafka::config::ClientConfig;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::producer::FutureProducer;

#[tokio::main]
async fn main() {
    init_logger(LevelFilter::Debug);
    info!("Starting");
    let args: Args = Args::parse();

    processor(args);
}

pub async fn processor(args: Args) {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.server", args.consumer)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "5000")
        .create()
        .expect("Unable to create consumer");

    let mut topics = Vec::<&str>::new();
    for topic in args.topic.iter() {
        topics.push(topic.as_str());
    }

    consumer
        .subscribe(&topics)
        .expect("Unable to subscribe to topic");

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.server", args.producer)
        .create()
        .expect("Unable to create producer");

    let mut stream_processor = consumer.stream();

    while let Some(message_result) = stream_processor.next().await {
        dbg!("HERE");
    }

}
