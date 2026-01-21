use std::{
    fs,
    path::Path,
    process::exit,
    sync::{Arc, Mutex},
};

use actix_cors::Cors;
use actix_web::{App, HttpServer, http, web};
use clap::Parser;
use data_orchestra_api::config::MetaAPIConfig;
use data_orchestra_api::{arguments::Arguments, routes::register::register_scope, state::State};
use data_orchestra_engine::{
    adapters::portainer::portainer::Portainer,
    config::Config,
    logger::init_logger,
    pipeline::{Pipeline, PipelineBuilder}
};
use tokio::sync::RwLock;
use tracing::{debug, error, info};
use data_orchestra_parser::traits::ToInternal;
use data_orchestra_parser::types::config::ExtConfig;

/// Main entry point of the Data Orchestra API
#[actix_web::main]
async fn main() {
    print_logo();

    // Load CLI arguments
    debug!("Loading CLI arguments");
    let args: Arguments = Arguments::parse();

    if fs::exists(&args.config_file).is_err() {
        panic!("Config file path {} invalid", args.config_file);
    }
    let config = fs::read_to_string(&args.config_file).expect("Unable to read config");

    // Load orchestrator configuration file
    let mut meta_config: MetaAPIConfig =
        toml::from_str(config.as_str()).expect("Unable to read config file");

    meta_config.combine(args);

    init_logger(meta_config.api.log_level);

    // Initialize API global state
    let mut state = State::default();

    // Load a architecture configuration file if present
    if let Some(paths) = meta_config.components_file.as_ref() {
        let mut main_config = Config::default();
        let portainer = Portainer::default();

        for path in paths.as_ref_vec().iter() {
            info!("Reading config file from {path}");
            let mut ext_config = ExtConfig::parse(&Path::new(path));

            // Transform attachable objects to configured objects
            let attach_objects = ext_config.extract_attachables();

            // Cascade uploader if specified in the orchestrator configuration
            if let Some(uploader) = meta_config.uploader.as_ref() {
                ext_config.set_uploader(uploader.clone());
            }

            let mut config = ext_config.to_internal();

            config.object.extend(attach_objects);

            config
                .agents
                .extend(portainer.create_agents(config.get_nodes()));

            main_config.combine(config);
        }

        info!("Total of {} components", main_config.get_number_of_components());

        state.config = RwLock::new(main_config);
        state.portainer = RwLock::new(portainer)
    }


    // Wrap in arc and mutex to allow passing to API while still being mutable for the pipeline
    let arc_state = Arc::new(Mutex::new(state));

    // Create async function of the pipeline
    let pipe_arc_state = arc_state.clone();
    let pipe = async move {
        if let Ok(state) = pipe_arc_state.clone().lock() {
            let mut config = state.config.write().await;
            let pipeline: Pipeline<(), String> = PipelineBuilder::default()
                .spawner_config(&mut config)
                .build()
                .expect("Unable to build pipeline");

            let result = pipeline.run();
            if let Err(error) = result {
                error!("{error}");
            }
        }
    };

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(arc_state.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "OPTIONS", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(register_scope())
    })
    .bind((meta_config.api.ip, meta_config.api.port))
    .unwrap()
    .run();

    // Join async server and async pipeline such that both are executed on start
    info!("Starting API");
    let _ = tokio::join!(server, pipe);
}

pub fn print_logo() {
    println!(
        r#"
    ____        __        ____            __              __
   / __ \____ _/ /_____ _/ __ \__________/ /_  ___  _____/ /__________ _          |\      _,,,---,,_
  / / / / __ `/ __/ __ `/ / / / ___/ ___/ __ \/ _ \/ ___/ __/ ___/ __ `/    ZZZzz /,`.-'`'    -.  ;-;;,_
 / /_/ / /_/ / /_/ /_/ / /_/ / /  / /__/ / / /  __(__  ) /_/ /  / /_/ /          |,4-  ) )-,_. ,\ (  `'-'
/_____/\__,_/\__/\__,_/\____/_/   \___/_/ /_/\___/____/\__/_/   \__,_/          '---''(_/--'  `-'\_)
    "#
    );
}
