use std::env::args;
use std::fs;
use std::future::Future;
use std::process::Output;
use std::{path::Path, sync::Arc};
use std::time::Duration;
use actix_web::{post, web, App, HttpResponse, HttpServer};
use kafka_producer::arguments::Arguments;
use log::{info, error, logger};
use rdkafka::error::KafkaError;
use rdkafka::message::OwnedMessage;
use rdkafka::{producer::{FutureProducer, FutureRecord}, ClientConfig};
use kafka_producer::logger::init_logger;
use serde_json::{from_str, json};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    info!("Starting");

    let config_path = Path::new("config.json");
    let config = fs::read_to_string(config_path).expect("Unable to read config file");
    dbg!(&config);
    let args: Arguments = from_str(config.as_str()).expect("Unable to parse config to struct");

    init_logger(args.level);

    if let Some(log_address) = args.logger.as_ref() {
        let producer = get_producer(log_address);
        let result = producer_send(&producer, "orchestra-log", &json!({ "from": "kafka-producer", "message": "Starting" }).to_string()).await;
        if let Err(error) = result {
            error!("{:?}", error);
        }
    }

    let api_port = args.api_port.clone();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::new(args.clone())))
            .service(produce)
    })
    .bind(("0.0.0.0", api_port))?
    .run()
    .await
}

fn get_producer(address: &String) -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", address)
        .create()
        .expect("Unable to create kafka producer")
}

pub async fn producer_send(
    producer: &FutureProducer,
    topic: &str,
    payload: &str,
) -> Result<(i32, i64), (KafkaError, OwnedMessage)> {
    producer
        .send(
            FutureRecord::to(topic)
                .payload(payload)
                .key(""),
            Duration::from_secs(60),
        )
        .await
}

#[post("/kafkaproducer")]
async fn produce(data: String, args: web::Data<Arc<Arguments>>) -> HttpResponse {
    let producer: FutureProducer = get_producer(&args.address);
    let mut logger: Option<FutureProducer> = None;
    if let Some(log_address) = args.logger.as_ref() {
        logger = Some(get_producer(log_address));
    }
    if args.topics.is_empty() {
        panic!("No topics provided for kafka");
    }

    for topic in args.topics.iter() {
        let delivery_status = producer_send(&producer, topic, &format!("{}", &data)).await;
        if let Err(error) = delivery_status {
            return HttpResponse::InternalServerError().body(format!("{:?}", error));
        }
        if let Some(logger) = logger.as_ref() {
            producer_send(logger, "orchestra-log", &json!({ "from": "kafka-producer", "message": format!("Sent to topic {}: {}", topic, data) }).to_string()).await;
        }
    }

    HttpResponse::Ok().finish()
}
