use clap::{command, Parser, ValueEnum};
use log::LevelFilter;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Arguments {
    #[arg(short = 'i', long)]
    pub interval: u64,

    #[arg(short = 't', long)]
    pub topic: Option<Vec<String>>,

    #[arg(short = 'p', long, value_name = "HOST:PORT")]
    pub producer: String,

    #[arg(short = 'l', long, default_value_t = LevelFilter::Info)]
    pub level: LevelFilter
}
