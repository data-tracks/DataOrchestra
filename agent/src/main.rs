mod routes;
mod manager;
mod logger;

use actix_cors::Cors;
use actix_web::{http, App, HttpServer};
use log::error;
use tracing::metadata::LevelFilter;
use crate::logger::init_logger;
use crate::routes::init::init_scope;
use crate::routes::logs::logs_scope;
use crate::routes::run::run_scope;

#[actix_web::main]
async fn main() {
    init_logger(LevelFilter::DEBUG);

    let res = HttpServer::new(move || {
        App::new()
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