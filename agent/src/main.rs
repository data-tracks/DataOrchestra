mod logger;
mod manager;
mod routes;

use std::sync::Arc;

use crate::manager::state_manager::StateManager;
use crate::routes::init::init_scope;
use crate::routes::logs::logs_scope;
use crate::routes::run::run_scope;
use crate::{logger::init_logger, manager::state_manager::Message};
use actix_cors::Cors;
use actix_web::{App, HttpServer, http, web};
use log::error;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, mpsc};
use tracing::metadata::LevelFilter;

#[actix_web::main]
async fn main() {
    init_logger(LevelFilter::DEBUG);
    let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel::<Message>(100);
    let state_manager = StateManager::new(rx);

    let state = Arc::new(tx);
    actix_web::rt::spawn(async move {
        state_manager.run().await;
    });

    let res = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "OPTIONS", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(logs_scope())
            .service(run_scope())
            .service(init_scope())
    })
    .bind(("localhost", 13260))
    .unwrap()
    .run()
    .await;

    if let Err(error) = res {
        error!("{}", error);
    }
}
