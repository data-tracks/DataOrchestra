use clap::{command, Parser, ValueEnum};
use log::LevelFilter;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[arg(short = 's', long)]
    pub stream_processor: Option<StreamProcessor>,

    #[arg(long)]
    pub consumer_topic: Vec<String>,

    #[arg(long)]
    pub producer_topic: Vec<String>,

    #[arg(short = 'c', long, value_name = "HOST:PORT")]
    pub consumer: String,

    #[arg(short = 'p', long, value_name = "HOST:PORT")]
    pub producer: String,

    #[arg(short = 'l', long, default_value_t = LevelFilter::Info)]
    pub level: LevelFilter
}

#[derive(ValueEnum, Parser, Debug, Clone)]
pub enum StreamProcessor {
    Kafka,
    Flink,
    Storm,
}

impl ToString for StreamProcessor {
    fn to_string(&self) -> String {
        match self {
            Self::Kafka => String::from("kafka"),
            Self::Flink => String::from("flink"),
            Self::Storm => String::from("storm"),
        }
    }
}
