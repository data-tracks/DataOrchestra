use std::{env, fs};
use std::io::{stdin, stdout, Write};
use std::net::{IpAddr, Ipv4Addr};
use std::path::Path;
use std::process::exit;
use std::str::FromStr;
use std::thread::{self};
use data_orchestra::core::adapters::{ping_node, ContainerType, DockerManager, Local, Portainer, Runner, Uploader};
use data_orchestra::core::generate::Generate;
use data_orchestra::core::object::Object;
use data_orchestra::core::process::Process;
use data_orchestra::core::store::Store;
use data_orchestra::core::types::Node;
use data_orchestra::interface::config::Config;
use data_orchestra::interface::generate::ExtGenerate;
use data_orchestra::interface::object::ExtObject;
use data_orchestra::interface::process::ExtProcess;
use data_orchestra::interface::store::ExtStore;
use data_orchestra::shared::traits::{Spawner, ToInternal, ToInternalVec};
use data_orchestra::shared::arguments::{Arguments, ARGS};
use data_orchestra::shared::ObjectTypes;
use data_orchestra::variables::variables::Variables;
use log::{info, warn, error};
use data_orchestra::logger::init_logger;
use data_orchestra::core::adapters::docker::{self};
use clap::Parser;

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
    dotenvy::dotenv().ok();
    let args: Arguments = Arguments::parse();

    if let Some(json_type) = args.generate_valid_json.as_ref() {
        match json_type {
            ObjectTypes::Generate => {
                let json = serde_json::to_string_pretty(&ExtGenerate::default()).expect("Unable to parse struct to json");
                println!("{}", json);
            },
            ObjectTypes::Store => {
                let json = serde_json::to_string_pretty(&ExtStore::default()).expect("Unable to parse struct to json");
                println!("{}", json);
            },
            ObjectTypes::Process => {
                let json = serde_json::to_string_pretty(&ExtProcess::default()).expect("Unable to parse struct to json");
                println!("{}", json);
            },
            ObjectTypes::Object => {
                let json = serde_json::to_string_pretty(&ExtObject::default()).expect("Unable to parse struct to json");
                println!("{}", json);
            }
        }

        exit(0);
    }
   
    init_logger(args.level);

    if args.file.is_none() {
        panic!("No config file specified. Please specify a config file with -f | --file  <path> argument or via the .env file key FILE=<path>");
    }

    if args.ssh_key.is_none() {
        warn!("No ssh key was provided. A ssh key is necessary when tasks are created on nodes. Please specify a ssh key with -s | --ssh-key <path> argument or via the .env file key SSH_KEY=<path>");
    }

    // Read config
    info!("Parsing config file");
    let config_path = Path::new(args.file.as_ref().unwrap());
    let config= fs::read_to_string(config_path).expect("Unable to read config file");

    // Read only variables from config into struct and transform config string to replace variables
    // with actual values before parsing the modified string to the config struct
    let variables: Variables = serde_json::from_str(config.as_str()).expect("Unable to parse config to struct");
    let config = variables.parse(config);

    let mut config: Config = serde_json::from_str(config.as_str()).expect("Unable to parse config to struct");
    info!("Finished parsing config file");

    let result = ARGS.set(args);
    if result.is_err() {
        panic!("Unable to setup CLI arguments as global static");
    }

    // Transform attachable objects to configured objects
    let attach_objects = config.extract_attachables();
    
    // Parse to internal structure and move everything out of config
    let mut objects: Vec<Object> = config.object.to_internal();
    let mut stores: Vec<Store> = config.store.to_internal();
    let mut processes: Vec<Process> = config.process.to_internal();
    let mut generates: Vec<Generate> = config.generate.to_internal();
    let mut portainer = config.portainer;

    objects.extend(attach_objects);

    health_check(&stores, &processes, &generates);

    // Start different tasks
    info!("Running tasks");
    
    thread::scope(|s| {
        for object in objects.iter_mut() {
            let _ = thread::Builder::new()
                .name("object".to_string())
                .spawn_scoped(s, || {
                    object.build();
            });
        }

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

    if ARGS.get().unwrap().remove_all {
        kill_containers(&stores, &processes, &generates);
    }

    if !ARGS.get().unwrap().no_portainer { 
        portainer.build(); 
    }

    pre_setup(&portainer, &stores, &processes, &generates);
   
    thread::scope(|s| {
        for object in objects.iter_mut() {
            let _ = thread::Builder::new()
                .name("object".to_string())
                .spawn_scoped(s, || {
                    object.setup();
            });
        }

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
        for object in objects.iter_mut() {
            let _ = thread::Builder::new()
                .name("object".to_string())
                .spawn_scoped(s, || {
                    object.deploy();
            });
        }

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
                            info!("Stopping all container {} on {}", &docker, &host);
                            let runner = ssh.to_box_runner();
                            let result = docker::api::stop_container(docker, &runner);
                            if let Err(error) = result {
                                error!("{}", error);
                            }
                        }
                    }
                }
            },
            "ka" | "kill-all" => {
                let nodes = get_nodes(&stores, &processes, &generates);
                for node in nodes {
                    if let Some(ref ssh) = node.ssh {
                        info!("Stopping all containers on {}", &node.host);
                        let runner = ssh.to_box_runner();
                        let result = docker::api::stop_containers(&runner);
                        if let Err(error) = result {
                            error!("{}", error);
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
ka      | kill-all      |                           | Kill all running docker containers
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

pub fn ping_nodes(node: &Node) {
    let result = ping_node(&node.host);
    if let Err(error) = result {
        panic!("[{}] {}", node.host, error);
    }
    info!("Node {} fully operational", node.host);
}

/// Perform health check on all remote nodes by ping
pub fn health_check(stores: &Vec<Store>, processes: &Vec<Process>, generates: &Vec<Generate>) {
    info!("Perfoming health check");

    for store in stores.iter() {
        if let Some(ref node) = store.object.node {
            ping_nodes(node);
        }
    }

    for process in processes.iter() {
        if let Some(ref node) = process.object.node {
            ping_nodes(node);
        }
    }

    for generate in generates.iter() {
        if let Some(ref node) = generate.object.node {
            ping_nodes(node);
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

                let existing_networks = docker::api::get_networks(&container.runner);
                if let Err(ref error) = existing_networks {
                    error!("{}", error);
                }
                let existing_networks = existing_networks.unwrap();

                if !existing_networks.contains(network) {
                    let result = docker::api::create_network(network, &container.runner);
                    if let Err(error) = result {
                        error!("{}", error);
                    }
                }
            }
            _ => ()
        } 
    }
}

pub fn kill_containers(stores: &Vec<Store>, processes: &Vec<Process>, generates: &Vec<Generate>) {
    let nodes = get_nodes(stores, processes, generates);
    for node in nodes {
        if let Some(ssh) = node.ssh.as_ref() {
            let runner = ssh.to_box_runner();
            info!("Removing all docker containers from {}", node.host);
            let result = docker::api::stop_containers(&runner);
            if let Err(error) = result {
                error!("{}", error);
            }
            let result = docker::api::delete_containers(&runner);
            if let Err(error) = result {
                error!("{}", error);
            }
        }
    }

    let runner = Local::new().to_box_runner();
    let result = docker::api::stop_containers(&runner);
    if let Err(error) = result {
        error!("{}", error);
    }
    let result = docker::api::delete_containers(&runner);
    if let Err(error) = result {
        error!("{}", error);
    }
}

pub fn pre_setup(portainer: &Portainer, stores: &Vec<Store>, processes: &Vec<Process>, generates: &Vec<Generate>) {
    info!("Performing pre setup");

    info!("Setting up docker networks");
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

    if !ARGS.get().unwrap().no_portainer {
        for node in nodes.iter() {
            portainer.create_agent(node);
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(portainer.add_agent(node));

            // Upload data to node
            if let Some(ref ssh) = node.ssh {
                let result = ssh.upload_directory("scripts", "/home/ubuntu/scripts");
                if let Err(error) = result {
                    error!("{}", error);
                }
            }
        }
    }

    for node in nodes.iter() {
        if let Some(ssh) = node.ssh.as_ref() {
            let result = docker::api::get_networks(&ssh.to_box_runner());
            if let Ok(networks) = result {
                if !networks.contains(&"orchestra".to_string()) {
                    let result = docker::api::create_network("orchestra", &ssh.to_box_runner());
                    if let Err(error) = result {
                        error!("{}", error);
                    }
                }
            }
        }
    }
}

pub fn cleanup(stores: &Vec<Store>, processes: &Vec<Process>, generates: &Vec<Generate>) {
    info!("Performing cleanup");

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
