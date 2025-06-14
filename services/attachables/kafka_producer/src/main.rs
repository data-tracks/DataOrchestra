use std::fs;
use std::{path::Path, sync::Arc};
use std::time::Duration;
use actix_web::{post, web, App, HttpResponse, HttpServer};
use kafka_producer::arguments::Arguments;
use log::info;
use rdkafka::{producer::{FutureProducer, FutureRecord}, ClientConfig};
use kafka_producer::logger::init_logger;
use serde_json::from_str;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    info!("Starting");

    let config_path = Path::new("config.json");
    let config = fs::read_to_string(config_path).expect("Unable to read config file");
    let args: Arguments = from_str(config.as_str()).expect("Unable to parse config to struct");

    init_logger(args.level);

    let api_port = args.api_port.clone();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::new(args.clone())))
            .service(produce)
    })
    .bind(("127.0.0.1", api_port))?
    .run()
    .await
}

#[post("/kafkaproducer")]
async fn produce(data: web::Path::<String>, args: web::Data<Arc<Arguments>>) -> HttpResponse {
    let data = data.into_inner();

    info!("Starting kafka producer");
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", &args.kafka_address)
        .create()
        .expect("Unable to create kafka producer");

    if args.topics.is_none() {
        panic!("No topics provided for kafka");
    }

    for topic in args.vec_topics.iter() {
        let delivery_status = producer
            .send(
                FutureRecord::to(topic)
                    .payload(&format!("{}", &data))
                    .key(""),
                Duration::from_secs(60),
            )
            .await;

        if let Err(error) = delivery_status {
            return HttpResponse::InternalServerError().body(format!("{:?}", error));
        }
    }

    HttpResponse::Ok().finish()
}
