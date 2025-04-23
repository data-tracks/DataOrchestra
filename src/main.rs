use std::fs::File;
use std::path::Path;
use std::thread::JoinHandle;
use log::{debug, info, LevelFilter};

use DataOrchester::docker;
use DataOrchester::logger::init_logger;
use DataOrchester::common::common_trait::Start;
use DataOrchester::types::amount::Amount;
use DataOrchester::types::config::Config;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    file: Option<String>,

    #[arg(short, long, default_value_t = LevelFilter::Info)]
    level: LevelFilter,

    #[arg(long = "remove_all", default_value_t = true)]
    remove_all: bool
}

fn main() {
    // Read starting arguments
    let args: Args = Args::parse();
   
    init_logger(args.level);

    if args.file == None {
        panic!("No config file specified. Please specify config with -f <path> argument");
    }

    if args.remove_all {
        docker::stop_all();
        docker::remove_all();
    }

    // Read config.json
    debug!("Loading config file {}", args.file.as_ref().unwrap());
    let config_path = Path::new(args.file.as_ref().unwrap());
    let config_file = File::open(config_path).expect("Unable to open config file");
    let config: Config = serde_json::from_reader(config_file).expect("Unable to parse config to struct");
    info!("Finished parsing config.json");

    let mut thread_pool: Vec<JoinHandle<()>> = Vec::new();

    // Start different tasks
    // Note: Task not referencable anymore as it is moved into `start`
    match config.store {
        Amount::None => (),
        Amount::Single(task) => {
            thread_pool.push(task.start());
        }
        Amount::Multiple(tasks) => {
            for task in tasks {
                thread_pool.push(task.start());
            }
        }
    }
    
    match config.process {
        Amount::None => (),
        Amount::Single(task) => {
            thread_pool.push(task.start());
        }
        Amount::Multiple(tasks) => {
            for task in tasks {
                thread_pool.push(task.start());
            }
        }
    }
     
    match config.generate {
        Amount::None => (),
        Amount::Single(task) => {
            thread_pool.push(task.start());
        }
        Amount::Multiple(tasks) => {
            for task in tasks {
                thread_pool.push(task.start());
            }
        }
    }

    // Wait for threads to finish
    for thread in thread_pool {
        let _ = thread.join();
    }
}
