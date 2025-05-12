use std::fs::File;
use std::path::Path;
use std::process::exit;
use std::thread::JoinHandle;
use data_orchestra::core::generate::Generate;
use data_orchestra::core::process::Process;
use data_orchestra::core::store::Store;
use data_orchestra::interface::config::Config;
use data_orchestra::shared::traits::{Spawner, ToInternal, ToInternalVec};
use log::{debug, info, LevelFilter};

use data_orchestra::logger::init_logger;
use data_orchestra::core::adapters::docker;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Config file location
    #[arg(short, long)]
    file: Option<String>,

    /// Logging level
    #[arg(short, long, default_value_t = LevelFilter::Info)]
    level: LevelFilter,

    /// Remove all running and stopped docker containers aswell as all networks
    #[arg(long = "remove_all", default_value_t = false)]
    remove_all: bool,

    /// Generate a valid config file 
    #[arg(long = "generate_valid_json", default_value_t = false)]
    generate_valid_json: bool
}

fn main() {
    print_logo();                                                             

    // Read starting arguments
    let args: Args = Args::parse();

    if args.generate_valid_json {
        println!("{}", serde_json::to_string_pretty(&Config::default()).unwrap());
        exit(-1);
    }
   
    init_logger(args.level);

    if args.file == None {
        panic!("No config file specified. Please specify config with -f <path> argument");
    }

    if args.remove_all {
        info!("Removing and deleting all docker containers");
        docker::stop_all_containers();
        docker::remove_all_containers();
        docker::remove_all_networks();
    }

    // Read config.json
    debug!("Loading config file {}", args.file.as_ref().unwrap());
    let config_path = Path::new(args.file.as_ref().unwrap());
    let config_file = File::open(config_path).expect("Unable to open config file");
    let config: Config = serde_json::from_reader(config_file).expect("Unable to parse config to struct");
    info!("Finished parsing config.json");

    let mut thread_pool: Vec<JoinHandle<()>> = Vec::new();

    // Parse to internal structure
    let stores: Vec<Store> = config.store.to_internal();
    let processes: Vec<Process> = config.process.to_internal();
    let generates: Vec<Generate> = config.generate.to_internal();

    prepare(&stores, &processes, &generates);

    // Start different tasks
    // Note: Task not referencable anymore as it is moved into `start`
    info!("Running tasks");

    let tasks = stores.iter_mut()
        .chain(processes.iter_mut())
        .chain()
   
    thread::scope(|s| {
        for store in stores
    })

    for store in stores {
        thread::scope(|s| {
            
        })
        thread_pool.push(store.build());
    }

    for process in processes {
        thread_pool.push(process.spawn());
    }

    for generate in generates {
        thread_pool.push(generate.spawn());
    }

    // Wait for threads to finish
    for thread in thread_pool {
        let _ = thread.join();
    }

    //cleanup(&stores, &processes, &generates);
}

pub fn print_logo() {
    println!(r#"
    ____        __        ____            __              __            
   / __ \____ _/ /_____ _/ __ \__________/ /_  ___  _____/ /__________ _
  / / / / __ `/ __/ __ `/ / / / ___/ ___/ __ \/ _ \/ ___/ __/ ___/ __ `/
 / /_/ / /_/ / /_/ /_/ / /_/ / /  / /__/ / / /  __(__  ) /_/ /  / /_/ / 
/_____/\__,_/\__/\__,_/\____/_/   \___/_/ /_/\___/____/\__/_/   \__,_/  
    "#);  
}

pub fn prepare(stores: &Vec<Store>, processes: &Vec<Process>, generates: &Vec<Generate>) {
    for store in stores.iter() {
    }

    for process in processes.iter() {
    }

    for generate in generates.iter() {
    }
}

pub fn cleanup(stores: &Vec<Store>, processes: &Vec<Process>, generates: &Vec<Generate>) {

}
