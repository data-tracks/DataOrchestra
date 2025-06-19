use std::sync::Arc;
use std::{env, fs};
use std::path::Path;
use std::process::exit;
use actix_web::rt::Runtime;
use data_orchestra::api::api::start_api;
use data_orchestra::api::state::State;
use data_orchestra::core::adapters::{ping_node, ContainerType, Local, Portainer, Runner, Uploader};
use data_orchestra::core::config::Config;
use data_orchestra::core::object::Object;
use data_orchestra::interface::config::ExtConfig;
use data_orchestra::shared::traits::ToInternal;
use data_orchestra::shared::arguments::Arguments;
use data_orchestra::shared::Spawner;
use data_orchestra::variables::variables::Variables;
use log::{info, warn, error};
use data_orchestra::logger::init_logger;
use data_orchestra::core::adapters::docker::{self};
use clap::Parser;

pub fn print_logo() {
    println!(r#"
    ____        __        ____            __              __                
   / __ \____ _/ /_____ _/ __ \__________/ /_  ___  _____/ /__________ _          |\      _,,,---,,_
  / / / / __ `/ __/ __ `/ / / / ___/ ___/ __ \/ _ \/ ___/ __/ ___/ __ `/    ZZZzz /,`.-'`'    -.  ;-;;,_
 / /_/ / /_/ / /_/ /_/ / /_/ / /  / /__/ / / /  __(__  ) /_/ /  / /_/ /          |,4-  ) )-,_. ,\ (  `'-'
/_____/\__,_/\__/\__,_/\____/_/   \___/_/ /_/\___/____/\__/_/   \__,_/          '---''(_/--'  `-'\_)
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
        json_type.print_json(); 
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
    let config= fs::read_to_string(config_path)
        .expect("Unable to read config file");

    // Read only variables from config into struct and transform config string to replace variables
    // with actual values before parsing the modified string to the config struct
    let variables: Variables = serde_json::from_str(config.as_str())
        .expect("Unable to parse config to struct");
    let ext_config_string = variables.parse(config);

    let mut ext_config: ExtConfig = serde_json::from_str(ext_config_string.as_str())
        .expect("Unable to parse config to struct");
    info!("Finished parsing config file");

    // Transform attachable objects to configured objects
    let attach_objects = ext_config.extract_attachables();
    let (mut config, mut portainer) = ext_config.to_internal();
    
    config.object.extend(attach_objects);

    // Inject portainer agents as objects
    let mut agents = Vec::<Object>::new();
    if args.portainer {
        for node in config.get_nodes() {
            agents.push(portainer.create_agent(node));
        }
    }

    config.object.extend(agents);

    for node in config.get_nodes_mut() {
        node.ssh_key = args.ssh_key.clone();
    }

    configuration_pipeline(&mut config, &mut portainer, &args);

    info!("Everything deployed. starting API.");
    let state = Arc::new(State::new(config, args));
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        start_api(state).await;
    });

    info!("Closing DataOrchestra");
}

/// Main creation pipeline of the programm. Processes and executes all main steps of the objects
pub fn configuration_pipeline(config: &mut Config, portainer: &mut Portainer, args: &Arguments) {
    info!("Starting configuration pipeline");

    health_check(&config);

    // pre build

    config.build();

    // post build

    if args.remove_all {
        kill_containers(&config);
    }

    if args.portainer { 
        portainer.build(); 
    }

    pre_setup(&portainer, &config, &args);

    config.setup();

    // post setup

    // pre deploy

    config.deploy();

    // post deploy
    
    info!("Finished configuration pipeline");
}

/// Check if programm is able to fully run. 
///
/// # Checks
///
/// - In an ansible enviroment
/// - Docker deamon running
pub fn viable_check() {
    match env::var("VIRTUAL_ENV") {
        Ok(_val) => {
            warn!("Python environment detected. Please ensure that the ansible package is contained in this environment");
        }
        Err(_) => {
            panic!("Not in an ansible environment. Please activate or create a python environment with ansible installed");
        }
    }

    let runner = Local::new();
    let result = runner.exec("docker info".to_string());
    if let Err(error) = result {
        panic!("Docker deamon not running. Make sure docker deamon is running before starting the programm. {}", error);
    }
}

/// Perform health check on all remote nodes by ping
pub fn health_check(config: &Config) {
    info!("Perfoming health check");

    let nodes = config.get_nodes();
    for node in nodes {
        let result = ping_node(&node.host);
        if let Err(error) = result {
            error!("[{}] {}", node.host, error);
        }
        info!("Node {} fully operational", node.host);
    }

    info!("Health check complete. All systems green");
}

pub fn setup_docker_networks(manager: &ContainerType) {
    for container in manager.containers_ref_vec() {
        let network = &container.config.network;
        if network.is_empty() {
            continue;
        } 

        let existing_networks = docker::api::get_networks(&*container.runner);
        if let Err(ref error) = existing_networks {
            error!("{}", error);
        }
        let existing_networks = existing_networks.unwrap();

        if !existing_networks.contains(network) {
            let result = docker::api::create_network(network, &*container.runner);
            if let Err(error) = result {
                error!("{}", error);
            }
        }
    }
}

pub fn kill_containers(config: &Config) {
    let nodes = config.get_nodes();
    for node in nodes {
        if let Some(ssh) = node.ssh.as_ref() {
            info!("Removing all docker containers from {}", node.host);
            let result = docker::api::stop_containers(ssh);
            if let Err(error) = result {
                error!("{}", error);
            }
            let result = docker::api::delete_containers(ssh);
            if let Err(error) = result {
                error!("{}", error);
            }
        }
    }

    let local = Local::new();
    let result = docker::api::stop_containers(&local);
    if let Err(error) = result {
        error!("{}", error);
    }
    let result = docker::api::delete_containers(&local);
    if let Err(error) = result {
        error!("{}", error);
    }
}

pub fn pre_setup(portainer: &Portainer, config: &Config, args: &Arguments) {
    info!("Performing pre setup");

    info!("Setting up docker networks");
    for store in config.store.iter() {
        setup_docker_networks(&store.object.docker_manager); 
    }

    for process in config.process.iter() {
        setup_docker_networks(&process.object.docker_manager); 
    }

    for generate in config.generate.iter() {
        setup_docker_networks(&generate.object.docker_manager); 
    }
   
    info!("Deploying portainer agent on nodes");
    let nodes = config.get_nodes();
    if !args.portainer {
        for node in nodes.iter() {
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
            let result = docker::api::get_networks(ssh);
            if let Ok(networks) = result {
                if !networks.contains(&"orchestra".to_string()) {
                    let result = docker::api::create_network("orchestra", ssh);
                    if let Err(error) = result {
                        error!("{}", error);
                    }
                }
            }
        }
    }
}
