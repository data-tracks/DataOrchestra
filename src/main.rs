use std::sync::Arc;
use std::time::Duration;
use std::{env, fs, thread};
use std::path::Path;
use std::process::exit;
use data_orchestra::core::adapters::{ping_node, ContainerType, Local, Portainer, Runner, Uploader};
use data_orchestra::core::config::Config;
use data_orchestra::core::object::Object;
use data_orchestra::core::types::Node;
use data_orchestra::interface::config::ExtConfig;
use data_orchestra::logger::init_logger;
use data_orchestra::shared::traits::ToInternal;
use data_orchestra::shared::arguments::Arguments;
use data_orchestra::shared::repeat_on_err_mut;
use data_orchestra::variables::variables::Variables;
use log::{debug, error, info, warn};
use data_orchestra::core::adapters::docker::{self};
use clap::Parser;
use data_orchestra::log_time;
use data_orchestra::shared::TIMESTAMPS;
use tokio::sync::RwLock;
use data_orchestra::api::api::start_api;
use data_orchestra::api::state::APIState;
use data_orchestra::core::traits::Spawner;
//////////////////////////////////////////////////////////////////////////////////////////////////
// This is the main entry point of the orchestrator. Here the configuration file is read, arguments
// are set and other related things. 
//////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    log_time!("Main start"); 

    print_logo();

    dotenvy::dotenv().ok();
    let args: Arguments = Arguments::parse();

    init_logger(args.level);

    info!("Starting DataOrchestra");
    viable_check();

    if let Some(json_type) = args.generate_valid_json.as_ref() {
        json_type.print_json(); 
        exit(0);
    }
   
    if args.file.is_none() {
        panic!("No config file specified. Please specify a config file with -f | --file  <path> argument or via the .env file key FILE=<path>");
    }

    if args.ssh_key.is_none() {
        warn!("No ssh key was provided. A ssh key is necessary when tasks are created on nodes. Please specify a ssh key with -s | --ssh-key <path> argument or via the .env file key SSH_KEY=<path>");
    }

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

    // If isolated objects are given, remove all object not matching given objects
    if let Some(isolated_objects) = &args.isolate && !isolated_objects.is_empty() {
        if !isolated_objects.is_empty() {
            let isolated_set: std::collections::HashSet<_> = isolated_objects.iter().collect();
            config.object.retain(|object| isolated_set.contains(&object.name));
            config.generate.retain(|generate| isolated_set.contains(&generate.object.name));
            config.process.retain(|process| isolated_set.contains(&process.object.name));
            config.store.retain(|store| isolated_set.contains(&store.object.name));
        }
    }

    // Create ssh session for all objects
    for node in config.get_object_nodes_mut() {
        node.ssh_key = args.ssh_key.clone();
        let result = repeat_on_err_mut(|| {
            node.set_ssh()
        }, 5, Some(Duration::from_secs(1)));
        if let Err(error) = result {
            error!("Unable to set ssh session for node {} ({error})", node.host);
        }
    }

    ////////////////////////////////////////////////////////
    // By here all components are set. No new ones are added. 
    ////////////////////////////////////////////////////////


    log_time!("Configuration start"); 
    let api_state = Arc::new(RwLock::new(APIState::default()));
    let clone_api_state = api_state.clone();
    let api_thread = thread::Builder::new()
        .name("api".to_string())
        .spawn(|| {
            info!("Starting API");
            let rt =  tokio::runtime::Runtime::new().unwrap();
            rt.block_on(start_api(clone_api_state));
        }).unwrap();

    configuration_pipeline(&mut config, &mut portainer, &args);

    log_time!("Configuration end"); 

    info!("Closing DataOrchestra");

    let mut timestamps = String::new();
    for time in TIMESTAMPS.lock().unwrap().iter() {
        let mut time_clone = time.clone();
        time_clone.push('\n');
        let time_clone = time_clone.as_str();
        timestamps.push_str(time_clone);
    }

    let amount_objects = format!("Amount of objects: {}", config.get_mut_spawners().count());

    println!("Stats:");
    println!("{timestamps}");
    println!("{amount_objects}");

    // Send config to API
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut state = api_state.write().await;
        state.set_config(config);
    });

    let _ = api_thread.join();
}

/// Main creation pipeline of the programm. Processes and executes all main steps of the objects
pub fn configuration_pipeline(config: &mut Config, portainer: &mut Portainer, args: &Arguments) {
    info!("Starting configuration pipeline");

    health_check(config);

    // pre build
    
    info!("Building");
    log_time!("Building start"); 

    config.build();

    info!("Finished building");
    log_time!("Building finished");

    // post build

    log_time!("Removing container");
    if args.remove_all {
        kill_containers(config.get_nodes());
    }

    exit(-1);

    log_time!("Building portainer");
    if args.portainer {
        portainer.build(); 
    }

    pre_setup(portainer, config, args);

    info!("Setting up");
    log_time!("Setup start");

    config.setup();

    info!("Finished setting up");
    log_time!("Setup finished");

    // post setup

    post_setup(portainer, config, args);

    // pre deploy

    info!("Deploying");
    log_time!("Deploy start");

    config.deploy();

    info!("Finished deploying");
    log_time!("Deploy finished");

    // post deploy
    
    info!("Finished configuration pipeline");
}

/// Check if program is able to fully run.
///
/// # Checks
///
/// - In an ansible environment
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
        panic!("Docker deamon not running. Make sure docker deamon is running before starting the program. ({error})");
    }
}

/// Perform health check on all remote nodes by ping
pub fn health_check(config: &Config) {
    info!("Performing health check");

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

/// Creates needed docker network for all objects
pub fn setup_docker_networks(manager: &ContainerType) {
    for container in manager.containers_ref_vec() {
        let network = &container.config.network;
        if network.is_empty() || network.eq("orchestra") {
            continue;
        } 

        let existing_networks = docker::api::get_networks(&*container.runner);
        if let Err(ref error) = existing_networks {
            error!("{error}");
        }
        let existing_networks = existing_networks.unwrap();

        if !existing_networks.contains(network) {
            let result = docker::api::create_network(network, &*container.runner);
            if let Err(error) = result {
                error!("{error}");
            }
        }
    }
}

/// Kill all containers on all nodes
pub fn kill_containers(nodes: Vec<&Node>) {
    thread::scope(|s| {
        for node in nodes {
            let _ = thread::Builder::new().spawn_scoped(s, || {
                if let Some(ssh) = node.ssh.as_ref() {
                    info!("Removing all docker containers from {}", node.host);
                    let result = docker::api::stop_containers(ssh);
                    if let Err(error) = result {
                        error!("{error}");
                    }
                    let result = docker::api::delete_containers(ssh);
                    if let Err(error) = result {
                        error!("{error}");
                    }
                }
                else {
                    error!("Node {} doesnt have ssh session", node.host);
                }
            });
        }

        let _ = thread::Builder::new().spawn_scoped(s, || {
            let local = Local::new();
            let result = docker::api::stop_containers(&local);
            if let Err(error) = result {
                error!("{error}");
            }
            let result = docker::api::delete_containers(&local);
            if let Err(error) = result {
                error!("{error}");
            }
        }); 
    });

    
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
   
    info!("Uploading scripts on nodes");
    let nodes = config.get_nodes();
    for node in nodes.iter() {
        // Upload data to node
        if let Some(ref ssh) = node.ssh {
            let result = ssh.upload_directory("scripts/", "scripts/");
            if let Err(error) = result {
                error!("{error}");
            }
        }
    }

    for node in nodes.iter() {
        if let Some(ssh) = node.ssh.as_ref() {
            let result = docker::api::get_networks(ssh);
            if let Ok(networks) = result && !networks.contains(&"orchestra".to_string()) {
                let result = docker::api::create_network("orchestra", ssh);
                if let Err(error) = result {
                    error!("{error}");
                }
            }
        }
    }

    let local = Local::new();
    let result = docker::api::get_networks(&local);
    if let Ok(networks) = result && !networks.contains(&"orchestra".to_string()) {
        let result = docker::api::create_network("orchestra", &local);
        if let Err(error) = result {
            error!("{error}");
        }
    }
}

pub fn post_setup(portainer: &Portainer, config: &Config, args: &Arguments) {
    let nodes = config.get_nodes();
    if args.portainer {
        for node in nodes.iter() {
            if let Some(ssh) = node.ssh.as_ref() {
                let containers = docker::api::get_container_names(ssh);
                if let Ok(containers) = containers {
                    if containers.contains(&"portainer_agent".to_string()) {
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        rt.block_on(portainer.add_agent(node));
                    }
                    else {
                        error!("No portainer agent on {}", node.host);
                    }
                }
                
            }
        }
    }
}

pub fn print_logo() {
    println!(r#"
    ____        __        ____            __              __                
   / __ \____ _/ /_____ _/ __ \__________/ /_  ___  _____/ /__________ _          |\      _,,,---,,_
  / / / / __ `/ __/ __ `/ / / / ___/ ___/ __ \/ _ \/ ___/ __/ ___/ __ `/    ZZZzz /,`.-'`'    -.  ;-;;,_
 / /_/ / /_/ / /_/ /_/ / /_/ / /  / /__/ / / /  __(__  ) /_/ /  / /_/ /          |,4-  ) )-,_. ,\ (  `'-'
/_____/\__,_/\__/\__,_/\____/_/   \___/_/ /_/\___/____/\__/_/   \__,_/          '---''(_/--'  `-'\_)
    "#);  
}
