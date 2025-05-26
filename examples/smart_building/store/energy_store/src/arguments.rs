use clap::{command, Parser};
use log::LevelFilter;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[arg(short, long, value_name = "HOST:PORT")]
    pub mongo_address: String,

    #[arg(short, long)]
    pub user: String,

    #[arg(short, long)]
    pub password: String,

    #[arg(long)]
    pub consumer_topic: Vec<String>,

    #[arg(short = 'c', long, value_name = "HOST:PORT")]
    pub consumer: String,

    #[arg(short = 'l', long, default_value_t = LevelFilter::Info)]
    pub level: LevelFilter
}
