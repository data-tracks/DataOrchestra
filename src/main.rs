use std::env;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::exit;
use std::thread::{self};
use data_orchestra::core::adapters::ping;
use data_orchestra::core::generate::Generate;
use data_orchestra::core::process::Process;
use data_orchestra::core::store::Store;
use data_orchestra::interface::config::Config;
use data_orchestra::shared::traits::{Spawner, ToInternal, ToInternalVec};
use log::{info, warn, LevelFilter};

use data_orchestra::logger::init_logger;
use data_orchestra::core::adapters::docker::{self};

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

pub fn print_logo() {
    println!(r#"
    ____        __        ____            __              __            
   / __ \____ _/ /_____ _/ __ \__________/ /_  ___  _____/ /__________ _
  / / / / __ `/ __/ __ `/ / / / ___/ ___/ __ \/ _ \/ ___/ __/ ___/ __ `/
 / /_/ / /_/ / /_/ /_/ / /_/ / /  / /__/ / / /  __(__  ) /_/ /  / /_/ / 
/_____/\__,_/\__/\__,_/\____/_/   \___/_/ /_/\___/____/\__/_/   \__,_/  
    "#);  
}

fn main() {
    info!("Starting DataOrchestra");
    viable_check();

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
        info!("Deleted all docker containers");
    }

    // Read config.json
    info!("Parsing config file");
    let config_path = Path::new(args.file.as_ref().unwrap());
    let config_file = File::open(config_path).expect("Unable to open config file");
    let config: Config = serde_json::from_reader(config_file).expect("Unable to parse config to struct");
    info!("Finished parsing config file");

    // Parse to internal structure
    let mut stores: Vec<Store> = config.store.to_internal();
    let mut processes: Vec<Process> = config.process.to_internal();
    let mut generates: Vec<Generate> = config.generate.to_internal();
 
    health_check(&stores, &processes, &generates);

    // Start different tasks
    // Note: Task not referencable anymore as it is moved into `start`
    info!("Running tasks");
    
    thread::scope(|s| {
        for store in stores.iter_mut() {
            let _ = thread::Builder::new()
                .name("store".to_string())
                .spawn_scoped(s, || {
                    store.build();
            });
        }

        for process in processes.iter_mut() {
            let _ = thread::Builder::new()
                .name("process".to_string())
                .spawn_scoped(s, || {
                    process.build();
            });
        }

        for generate in generates.iter_mut() {
            let _ = thread::Builder::new()
                .name("generate".to_string())
                .spawn_scoped(s, || {
                    generate.build();
            });
        }
    });

    pre_setup(&stores, &processes, &generates);
   
    thread::scope(|s| {
        for store in stores.iter_mut() {
            let _ = thread::Builder::new()
                .name("store".to_string())
                .spawn_scoped(s, || {
                    store.setup();
            });
        }

        for process in processes.iter_mut() {
            let _ = thread::Builder::new()
                .name("process".to_string())
                .spawn_scoped(s, || {
                    process.setup();
            });
        }

        for generate in generates.iter_mut() {
            let _ = thread::Builder::new()
                .name("generate".to_string())
                .spawn_scoped(s, || {
                    generate.setup();
            });
        }
    });

    thread::scope(|s| {
        for store in stores.iter_mut() {
            let _ = thread::Builder::new()
                .name("store".to_string())
                .spawn_scoped(s, || {
                    store.deploy();
            });
        }

        for process in processes.iter_mut() {
            let _ = thread::Builder::new()
                .name("process".to_string())
                .spawn_scoped(s, || {
                    process.deploy();
            });
        }

        for generate in generates.iter_mut() {
            let _ = thread::Builder::new()
                .name("generate".to_string())
                .spawn_scoped(s, || {
                    generate.deploy();
            });
        }
    });

    let mut do_cleanup: bool = true;

    info!("Everything deployed. Enabling CLI.");

    // Enter main loop of programm. Infinite loop which allows the user to communicate with
    // programm and remote entities. 
    loop {
        let mut s=String::new();
        print!("Enter command: ");
        let _ = stdout().flush();
        let _ = stdin().read_line(&mut s);

        match s.as_str() {
            "nc" | "no-cleanup" => { 
                do_cleanup = false; 
            }
            "hc" | "health-check" => {
                health_check(&stores, &processes, &generates);
            },
            "i" | "info" => {
                println!("Stores: {}", stores.len());
                println!("Process: {}", processes.len());
                println!("Generate: {}", generates.len());
            }
            "q" | "quit" => break,
            "h" | "help" | _ => {
                println!(
                    r#"
    DataOrchestra   |
    ----------------|

    short   | long          | explanation 
    -------------------------------------------------------------------------
    h       | help          | Get explanation of possible commands
    i       | info          | Get info of produced system
    q       | quit          | Quit programm and perform cleanup
    nc      | no-cleanup    | Perform no cleanup
    hc      | health-check  | Perform a health check on remote entities
                    "#
                    );
            }
        }
    }
        
    if do_cleanup {
        cleanup(&stores, &processes, &generates);
    }

    info!("Closing DataOrchestra");
}

pub fn viable_check() {
    match env::var("VIRTUAL_ENV") {
        Ok(_val) => {
            warn!("Python environment detected. Please ensure that the ansible package is contained in this environment");
        }
        Err(_) => {
            panic!("Not in an ansible environment. Please activate or create a python environment with ansible installed");
        }
    }
}

pub fn health_check(stores: &Vec<Store>, processes: &Vec<Process>, generates: &Vec<Generate>) {
    info!("Perfoming health check");

    for store in stores.iter() {
        if let Some(ref node) = store.object.node {
            let result = ping::ping(&node.address.ip);
            if let Err(error) = result {
                panic!("[{}] {}", node.address.ip, error);
            }
        }
    }

    for process in processes.iter() {
        if let Some(ref node) = process.object.node {
            let result = ping::ping(&node.address.ip);
            if let Err(error) = result {
                panic!("[{}] {}", node.address.ip, error);
            }
        }
    }

    for generate in generates.iter() {
        if let Some(ref node) = generate.object.node {
            let result = ping::ping(&node.address.ip);
            if let Err(error) = result {
                panic!("[{}] {}", node.address.ip, error);
            }
        }
    }

    info!("Health check complete. All systems green");
}

pub fn pre_setup(stores: &Vec<Store>, processes: &Vec<Process>, generates: &Vec<Generate>) {
    for store in stores.iter() {
        if let Some(ref manager) = store.object.docker_manager {
            for (_, container) in manager.containers.iter() {
                let network = &container.config.network;
                if network.is_empty() {
                    break;
                } 

                let existing_networks = docker::get_networks();
                if !existing_networks.contains(network) {
                    let result = docker::create_network(network);
                    if let Err(error) = result {
                        panic!("Unable to create docker network. {}", error);
                    }
                }
            }
        }
    }

    for process in processes.iter() {
        if let Some(ref manager) = process.object.docker_manager {
            for (_, container) in manager.containers.iter() {
                let network = &container.config.network;
                if network.is_empty() {
                    break;
                } 

                let existing_networks = docker::get_networks();
                if !existing_networks.contains(network) {
                    let result = docker::create_network(network);
                    if let Err(error) = result {
                        panic!("Unable to create docker network. {}", error);
                    }
                }
            }
        }
    }

    for generate in generates.iter() {
        if let Some(ref manager) = generate.object.docker_manager {
            for (_, container) in manager.containers.iter() {
                let network = &container.config.network;
                if network.is_empty() {
                    break;
                } 

                let existing_networks = docker::get_networks();
                if !existing_networks.contains(network) {
                    let result = docker::create_network(network);
                    if let Err(error) = result {
                        panic!("Unable to create docker network. {}", error);
                    }
                }
            }
        }
    }
}

pub fn cleanup(stores: &Vec<Store>, processes: &Vec<Process>, generates: &Vec<Generate>) {
    info!("Performing cleanup");

    for _store in stores.iter() {
    }

    for _process in processes.iter() {
    }

    for _generate in generates.iter() {
    }

    info!("Cleanup complete");
}
