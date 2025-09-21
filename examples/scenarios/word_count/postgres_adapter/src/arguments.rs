use clap::{command, Parser};
use log::LevelFilter;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Kafka topic from which data is consumed
    #[arg(long)]
    pub topic: Vec<String>,

    /// Connection from which data is consumed
    #[arg(short = 'c', long, value_name = "HOST:PORT")]
    pub consumer: String,

    /// Logging level
    #[arg(short = 'l', long, default_value_t = LevelFilter::Info)]
    pub level: LevelFilter
}
