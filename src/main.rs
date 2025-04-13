use std::env;
use std::fs::File;
use std::net::{IpAddr, Ipv4Addr};
use std::path::Path;
use std::thread::JoinHandle;
use log::{debug, info, LevelFilter};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::io::Write;

use DataOrchester::logger::init_logger;

use DataOrchester::common::common_trait::Start;
use DataOrchester::generate::generate_struct::Generate;
use DataOrchester::process::process_struct::Process;
use DataOrchester::store::store_struct::Store;

use DataOrchester::types::address::Address;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Amount<T> {
    Single(T),
    Multiple(Vec<T>)
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Node {
    address: Address
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
struct Config {
    process: Amount<Process>,
    generate: Amount<Generate>,
    store: Amount<Store>
}

fn main() {
    // Read starting arguments
    let args: Vec<String> = env::args().collect();
    
    initialise_logger(&args);
    
    // Read config.json
    let config_path = Path::new("config.json");
    let config_file = File::open(config_path).expect("Unable to open config file");
    let mut config: Config = serde_json::from_reader(config_file).expect("Unable to parse config to struct");
    info!("Finished parsing config.json");

    // Get amount of docker containers to assign ports
    let mut docker_amount: usize = 3;

    let addresses: Vec<Address> = gen_unique_address(docker_amount);
    let mut current_address: usize = 0;
    
    let mut thread_pool: Vec<JoinHandle<()>> = Vec::new();

    // Start different tasks
    // Note: Task reference not referencable anymore
    match config.store {
        Amount::Single(mut task) => {
            task.docker.as_mut().unwrap().address = addresses[current_address]; 
            current_address += 1;
            thread_pool.push(task.start());
        }
        Amount::Multiple(tasks) => {
            for mut task in tasks {
                task.docker.as_mut().unwrap().address = addresses[current_address]; 
                current_address += 1;
                thread_pool.push(task.start());
            }
        }
    }
    
    match config.process {
        Amount::Single(mut task) => {
            task.docker.as_mut().unwrap().address = addresses[current_address];
            current_address += 1;
            thread_pool.push(task.start());
        }
        Amount::Multiple(tasks) => {
            for mut task in tasks {
                task.docker.as_mut().unwrap().address = addresses[current_address];
                current_address += 1;
                thread_pool.push(task.start());
            }
        }
    }
     
    match config.generate {
        Amount::Single(mut task) => {
            task.docker.as_mut().unwrap().address = addresses[current_address];
            current_address += 1;
            thread_pool.push(task.start());
        }
        Amount::Multiple(tasks) => {
            for mut task in tasks {
                task.docker.as_mut().unwrap().address = addresses[current_address];
                current_address += 1;
                thread_pool.push(task.start());
            }
        }
    }
    for thread in thread_pool {
        let _ = thread.join();
    }
}

pub fn initialise_logger(args: &Vec<String>) {
    let mut log_level: LevelFilter = LevelFilter::Error;

    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == "-l" {
            if let Some(level) = iter.next() {
                log_level = match level.as_str() {
                    "info" => LevelFilter::Info,
                    "warn" => LevelFilter::Warn,
                    "error" => LevelFilter::Error,
                    "debug" => LevelFilter::Debug,
                    "trace" => LevelFilter::Trace,
                    "off" => LevelFilter::Off,
                    _ => log_level,
                };
            }
        }
    }

    init_logger(log_level);
}

pub fn gen_unique_address(amount: usize) -> Vec<Address> {
    let mut addresses: Vec<Address> = Vec::<Address>::new();
    for _ in 0..amount {
        let port = rand::thread_rng().gen_range(100..10000);
//        let internal_port = rand::thread_rng().gen_range(10..100);
        addresses.push(Address {
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port,
            internal_port: 22
        }); 
    }

    addresses
}

