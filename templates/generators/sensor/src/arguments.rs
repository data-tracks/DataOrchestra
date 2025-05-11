use clap::{command, Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[arg(short = 's', long)]
    pub stream_processor: Option<StreamProcessor>,

    #[arg(short = 'i', long)]
    pub interval: u64,

    #[arg(short = 't', long)]
    pub topic: Option<Vec<String>>,

    #[arg(short = 'a', long, value_name = "HOST:PORT")]
    pub address: String 
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
