use std::env;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::net::{IpAddr, Ipv4Addr};
use std::path::Path;
use std::process::exit;
use std::str::FromStr;
use std::sync::OnceLock;
use std::thread::{self};
use data_orchestra::core::adapters::{ping, ContainerType, DockerManager, Portainer, Runner};
use data_orchestra::core::generate::Generate;
use data_orchestra::core::process::Process;
use data_orchestra::core::store::Store;
use data_orchestra::core::types::Node;
use data_orchestra::interface::config::Config;
use data_orchestra::shared::traits::{Spawner, ToInternal, ToInternalVec};
use log::{info, warn, error, LevelFilter};

use data_orchestra::logger::init_logger;
use data_orchestra::core::adapters::docker::{self};

use clap::Parser;

static ARGS: OnceLock<Args> = OnceLock::new();

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
    generate_valid_json: bool,

    /// Skip the portainer manager setup
    #[arg(long = "no_portainer", default_value_t = false)]
    no_portainer: bool,

    // Authorized ssh key for remote connections
    //#[arg(short, long)]
    //ssh_key: String 
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
        //docker::stop_all_containers();
        //docker::remove_all_containers();
        //docker::remove_all_networks();
        info!("Deleted all docker containers");
    }

    // Read config.json
    info!("Parsing config file");
    let config_path = Path::new(args.file.as_ref().unwrap());
    let config_file = File::open(config_path).expect("Unable to open config file");
    let config: Config = serde_json::from_reader(config_file).expect("Unable to parse config to struct");
    info!("Finished parsing config file");

    let result = ARGS.set(args);
    if result.is_err() {
        panic!("Unable to setup CLI arguments as global static");
    }

    // Parse to internal structure and move everything out of config
    let mut stores: Vec<Store> = config.store.to_internal();
    let mut processes: Vec<Process> = config.process.to_internal();
    let mut generates: Vec<Generate> = config.generate.to_internal();
    let mut portainer = config.portainer;

    health_check(&stores, &processes, &generates);

    portainer.build(); 

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

    pre_setup(&portainer, &stores, &processes, &generates);
   
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
        s = s.replace("\n", "");
        let mut command = s.trim();
        let mut parameters = "";

        // If command containes whitespace after trim, it holds parameters
        if command.contains(" ") {
            let result = s.split_once(" ");

            if result.is_none() {
                continue;
            }

            (command, parameters) = result.unwrap();
        }

        match command {
            "nc" | "no-cleanup" => { 
                do_cleanup = false; 
            }
            "hc" | "health-check" => {
                health_check(&stores, &processes, &generates);
            },
            "i" | "info" => {
                println!("Stores: {}", stores.len());
                for store in stores.iter() {
                    let mut store_format = String::new();
                    if let Some(ref node) = store.object.node {
                        store_format = format!("{store_format}-ip: {}\n", node.host);
                    }
                    println!("{}", store_format);
                }
                
                println!("Processes: {}", processes.len());
                for process in processes.iter() {
                    let mut process_format = String::new();
                    if let Some(ref node) = process.object.node {
                        process_format = format!("{process_format}-ip: {}\n", node.host);
                    }
                    println!("{}", process_format);
                }

                println!("Generates: {}", generates.len());
                for generate in generates.iter() {
                    let mut generate_format = String::new();
                    if let Some(ref node) = generate.object.node {
                        generate_format = format!("{generate_format}-ip: {}\n", node.host);
                    }
                    println!("{}", generate_format);
                }
            },
            "k" | "kill" => {
                if parameters.is_empty() {
                    warn!("Please provide node ip and docker name to kill docker process");
                    continue;
                }
                let parameters = parameters.split_once(" ");
                if parameters.is_none() {
                    warn!("Please provide node ip and docker name to kill docker process");
                    continue;
                }
                
                let (host, docker) = parameters.unwrap();
                let nodes = get_nodes(&stores, &processes, &generates);
                for node in nodes {
                    if node.host.eq(&IpAddr::V4(Ipv4Addr::from_str(host).unwrap())) {
                        if let Some(ref ssh) = node.ssh {
                            let runner = Box::new(ssh.clone()) as Box<dyn Runner + Send>;
                            let result = docker::api::stop_container(docker, Some(&runner));
                            if let Err(error) = result {
                                error!("{}", error);
                            }
                        }
                    }
                }

            }
            "q" | "quit" => break,
            "h" | "help" | _ => {
                println!(
                    r#"
    ____        __        ____            __              __                
   / __ \____ _/ /_____ _/ __ \__________/ /_  ___  _____/ /__________ _          |\      _,,,---,,_
  / / / / __ `/ __/ __ `/ / / / ___/ ___/ __ \/ _ \/ ___/ __/ ___/ __ `/    ZZZzz /,`.-'`'    -.  ;-;;,_
 / /_/ / /_/ / /_/ /_/ / /_/ / /  / /__/ / / /  __(__  ) /_/ /  / /_/ /          |,4-  ) )-,_. ,\ (  `'-'
/_____/\__,_/\__/\__,_/\____/_/   \___/_/ /_/\___/____/\__/_/   \__,_/          '---''(_/--'  `-'\_)

short   | long          | parameters                | description 
-------------------------------------------------------------------------
h       | help          |                           | Get explanation of possible commands
i       | info          |                           | Get info of produced system
q       | quit          |                           | Quit programm and perform cleanup
k       | kill          | <node ip> <docker name>   | Kill a specific docker container
nc      | no-cleanup    |                           | Perform no cleanup
hc      | health-check  |                           | Perform a health check on remote entities
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

/// Check if programm is able to fully run. 
///
/// # Checks
///
/// - In an ansible enviroment
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

/// Perform health check on all remote nodes by ping
pub fn health_check(stores: &Vec<Store>, processes: &Vec<Process>, generates: &Vec<Generate>) {
    info!("Perfoming health check");

    for store in stores.iter() {
        if let Some(ref node) = store.object.node {
            let result = ping::ping(&node.host);
            if let Err(error) = result {
                panic!("[{}] {}", node.host, error);
            }
            info!("Node-{} fully operational", node.host);
        }
    }

    for process in processes.iter() {
        if let Some(ref node) = process.object.node {
            let result = ping::ping(&node.host);
            if let Err(error) = result {
                panic!("[{}] {}", node.host, error);
            }
            info!("Node-{} fully operational", node.host);
        }
    }

    for generate in generates.iter() {
        if let Some(ref node) = generate.object.node {
            let result = ping::ping(&node.host);
            if let Err(error) = result {
                panic!("[{}] {}", node.host, error);
            }
            info!("Node-{} fully operational", node.host);
        }
    }

    info!("Health check complete. All systems green");
}

pub fn setup_docker_networks(manager: &DockerManager) {
    for (_, item) in manager.containers.iter() {
        match item {
            ContainerType::Container(container) => {
                let network = &container.config.network;
                if network.is_empty() {
                    continue;
                } 

                if let Some(ref runner) = container.runner {
                    let existing_networks = docker::api::get_networks(Some(runner));
                    if let Err(ref error) = existing_networks {
                        error!("{}", error);
                    }
                    let existing_networks = existing_networks.unwrap();

                    if !existing_networks.contains(network) {
                        let result = docker::api::create_network(network, Some(runner));
                        if let Err(error) = result {
                            error!("{}", error);
                        }
                    }
                }
                else {
                    let existing_networks = docker::api::get_networks(None);
                    if let Err(ref error) = existing_networks {
                        error!("{}", error);
                    }
                    let existing_networks = existing_networks.unwrap();

                    if !existing_networks.contains(network) {
                        let result = docker::api::create_network(network, None);
                        if let Err(error) = result {
                            error!("{}", error);
                        }
                    }
                }
            }
            _ => ()
        } 
    }
}

pub fn pre_setup(portainer: &Portainer, stores: &Vec<Store>, processes: &Vec<Process>, generates: &Vec<Generate>) {
    info!("Performing pre setup");

    info!("Copying necessary scripts");
    for store in stores.iter() {
        if let Some(ref manager) = store.object.docker_manager {
            setup_docker_networks(manager); 
        }
    }

    for process in processes.iter() {
        if let Some(ref manager) = process.object.docker_manager {
            setup_docker_networks(manager); 
        }
    }

    for generate in generates.iter() {
        if let Some(ref manager) = generate.object.docker_manager {
            setup_docker_networks(manager); 
        }
    }
   
    info!("Deploying portainer agent on nodes");
    let nodes = get_nodes(stores, processes, generates);

    

    for node in nodes {
        if ARGS.get().unwrap().remove_all {
            if let Some(ssh) = node.ssh.as_ref() {
                let runner = Box::new(ssh.clone()) as Box<dyn Runner + Send>;
                info!("Removing all docker containers from {}", node.host);
                let result = docker::api::stop_containers(Some(&runner));
                if let Err(error) = result {
                    error!("{}", error);
                }
                let result = docker::api::delete_containers(Some(&runner));
                if let Err(error) = result {
                    error!("{}", error);
                }
            }
        }
        portainer.create_agent(node);
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _ = rt.block_on(portainer.add_agent(node));
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

pub fn get_nodes<'a>(stores: &'a Vec<Store>, processes: &'a Vec<Process>, generates: &'a Vec<Generate>) -> Vec<&'a Node> {
    let mut nodes = Vec::<&'a Node>::new();

    for store in stores.iter() {
        if let Some(ref node) = store.object.node {
            if !nodes.iter().any(|x| x.host.eq(&node.host)) {
                nodes.push(node);
            }
        }
    }

    for process in processes.iter() {
        if let Some(ref node) = process.object.node {
            if !nodes.iter().any(|x| x.host.eq(&node.host)) {
                nodes.push(node);
            }
        }
    }

    for generate in generates.iter() {
        if let Some(ref node) = generate.object.node {
            if !nodes.iter().any(|x| x.host.eq(&node.host)) {
                nodes.push(node);
            }
        }
    } 

    nodes
}
