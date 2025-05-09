use rdkafka::{producer::BaseProducer, ClientConfig};

fn main() {
    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.server", "kafka:9092")
        .create()
        .expect("Unable to create producer");

    producer.send(record)
}
