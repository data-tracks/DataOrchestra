use clap::Parser;
use log::LevelFilter;

#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct Arguments {
    #[arg(short, long, default_value_t = 8080)]
    #[arg(env = "API_PORT")]
    pub api_port: u16,

    #[arg(short, long)]
    #[arg(env = "TOPIC")]
    pub topics: Option<String>,

    #[arg(short, long, value_name = "HOST:PORT")]
    #[arg(env = "KAFKA_ADDRESS")]
    pub kafka_address: String,

    #[arg(short, long, default_value_t = LevelFilter::Info)]
    #[arg(env = "LEVEL")]
    pub level: LevelFilter,

    #[arg(skip)]
    pub vec_topics: Vec<String>
}
