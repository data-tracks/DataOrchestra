use std::sync::{Arc, Mutex};

use crate::manager::state_manager::Message;
use crate::manager::state_manager::{Master, StateManager};
use actix_web::post;
use actix_web::web;
use actix_web::{HttpResponse, Scope};
use log::info;
use tokio::sync::mpsc::Sender;

pub fn init_scope() -> Scope {
    web::scope("/init").service(post_init)
}

#[post("")]
async fn post_init(
    state_sender: web::Data<Arc<Sender<Message>>>,
    json: web::Json<Master>,
) -> Result<HttpResponse, actix_web::Error> {
    let master: Master = json.into_inner();
    let result = state_sender.blocking_send(Message {
        message_type: crate::manager::state_manager::MessageType::State,
        message: serde_json::to_string(&master).unwrap(),
    });

    if let Err(error) = result {
        //return Err(HttpResponse::InternalServerError().finish());
    }

    Ok(HttpResponse::Ok().finish())
}
