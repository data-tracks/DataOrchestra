use clap::{command, Parser};
use log::LevelFilter;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// MongoDB database
    #[arg(short, long)]
    pub database: String,

    /// MongoDB collection
    #[arg(long)]
    pub collection: String,

    /// MongoDB host:port connection
    #[arg(short, long, value_name = "HOST:PORT")]
    pub mongo_address: String,

    /// MongoDB username
    #[arg(short, long)]
    pub user: String,

    /// MongoDB userpassword
    #[arg(short, long)]
    pub password: String,

    /// Kafka topic from which data is consumed
    #[arg(long)]
    pub consumer_topic: Vec<String>,

    /// Connection from which data is consumed
    #[arg(short = 'c', long, value_name = "HOST:PORT")]
    pub consumer: String,

    /// Logging level
    #[arg(short = 'l', long, default_value_t = LevelFilter::Info)]
    pub level: LevelFilter
}
