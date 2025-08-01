use clap::Parser;
use log::LevelFilter;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Arguments {
    #[arg(short = 'i', long)]
    pub interval: u64,

    #[arg(short = 'l', long, default_value_t = LevelFilter::Info)]
    pub level: LevelFilter,

    #[arg(short = 'a', long, value_name = "HOST:PORT")]
    pub address: String
}
