use std::sync::Arc;
use std::time::Duration;
use actix_web::{post, web, App, HttpResponse, HttpServer};
use clap::Parser;
use kafka_producer::arguments::Arguments;
use log::{info, LevelFilter};
use rdkafka::{producer::{FutureProducer, FutureRecord}, ClientConfig};
use kafka_producer::logger::init_logger;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_logger(LevelFilter::Debug);
    info!("Starting");

    dotenvy::dotenv().ok();
    let mut args: Arguments = Arguments::parse();

    let mut vec_topics = Vec::<String>::new();
    if let Some(topics) = args.topics.as_ref() {
        if topics.contains("[") {
            let topics = topics.replace("[", "").replace("]", "");
            for topic in topics.split(",") {
                vec_topics.push(topic.to_string().replace(" ", ""));
            }
        }
    }
    args.vec_topics = vec_topics;

    dbg!(&args);

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
