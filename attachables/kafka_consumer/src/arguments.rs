use clap::Parser;
use log::LevelFilter;

#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct Arguments {
    #[arg(short, long)]
    #[arg(env = "API_PORT")]
    pub api_port: u16,

    #[arg(short = 't', long)]
    #[arg(env = "TOPIC")]
    pub topic: Option<Vec<String>>,

    #[arg(short = 'a', long, value_name = "HOST:PORT")]
    #[arg(env = "KAFKA_ADDRESS")]
    pub kafka_address: String,

    #[arg(short = 'l', long)]
    #[arg(env = "LEVEL")]
    pub level: LevelFilter
}
