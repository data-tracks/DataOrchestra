use std::{net::IpAddr, sync::Arc};
use actix_cors::Cors;
use actix_web::{get, http::{self, header::ContentType}, post, put, rt::System, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use log::error;

use crate::{core::adapters::{async_ping_node, docker, StateTypes}, state::State};

pub async fn start_api(state: Arc<State>) {
    let _ = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()  // For development only, allows all origins
                    .allowed_methods(vec!["GET", "POST", "OPTIONS", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
            )
            .app_data(web::Data::new(state.clone()))
            .service(get_active)
            .service(get_healthcheck)
            .service(put_kill)
            .service(put_killall)
            .service(post_broadcast)
            .service(get_broadcast)
            .service(metric_store)
            .service(metric_process)
            .service(metric_generate)
            .service(metric_object)
    })
    .bind(("127.0.0.1", 5000)).unwrap()
    .run()
    .await;
}

#[derive(Deserialize, Serialize, Clone)]
pub struct BroadcastMessage {
    pub from: String,
    pub message: String
}

#[get("orchestra/active")]
pub async fn get_active() -> impl Responder {
    HttpResponse::Ok()
}

#[get("orchestra/healthcheck")]
pub async fn get_healthcheck(state: web::Data<Arc<State>>) -> impl Responder {
    let mut health_data = Vec::new();

    let nodes = state.get_config().get_all_nodes();
    for node in nodes {
        let result = async_ping_node(&node.host).await;
        health_data.push(
            json!(
                {
                    "type": "node",
                    "name": "node",
                    "host": node.host, 
                    "healthy": result.is_ok()
                }
            )
        );

        let ssh = node.get_ssh(state.get_args().ssh_key.as_ref().unwrap());
        let result = docker::api::get_container_data(&ssh);
        if let Err(error) = ssh.disconnect() {
            error!("{}", error);
        } 
        if let Ok(containers) = result {
            for container in containers {
                health_data.push(
                    json!(
                        {
                            "type": "container",
                            "name": container.names,
                            "host": node.host,
                            "healthy": container.state.eq(&StateTypes::Running)
                        }
                    )
                );
            }  
        }
        else if let Err(error) = result {
            return HttpResponse::InternalServerError()
                .content_type(ContentType::plaintext())
                .body(error);
        }
    }

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(health_data)
}

#[put("orchestra/kill/{host}/{name}")]
pub async fn put_kill(data: web::Path<(IpAddr, String)>, state: web::Data<Arc<State>>) -> impl Responder {
    let (host, name) = data.into_inner();
    let nodes = state.get_config().get_all_nodes();
    for node in nodes {
        if node.host.eq(&host) {
            let ssh = node.get_ssh(state.get_args().ssh_key.as_ref().unwrap());
            let result = docker::api::kill_container(&ssh, &name);
            if let Err(error) = ssh.disconnect() {
                error!("{}", error);
            } 
            if let Err(error) = result {
                return HttpResponse::InternalServerError()
                    .content_type(ContentType::plaintext())
                    .body(error)
            }
        }
    }

    HttpResponse::Ok().into()
}

#[put("orchestra/killall")]
pub async fn put_killall(state: web::Data<Arc<State>>) -> impl Responder {
    let nodes = state.get_config().get_all_nodes();
    for node in nodes {
        let ssh = node.get_ssh(state.get_args().ssh_key.as_ref().unwrap());
        let result = docker::api::kill_containers(&ssh);
        if let Err(error) = ssh.disconnect() {
            error!("{}", error);
        } 
        if let Err(error) = result {
            return HttpResponse::InternalServerError()
                .content_type(ContentType::plaintext())
                .body(error);
        }
    }

    HttpResponse::Ok().into()
}

#[post("orchestra/broadcast")]
pub async fn post_broadcast(body: web::Json<BroadcastMessage>, state: web::Data<Arc<State>>) -> HttpResponse {
    if let Ok(writer) = state.messages.write().as_mut() {
        writer.push(body.into_inner());
    }
    else {
        return HttpResponse::InternalServerError().into()
    }

    HttpResponse::Ok().into()
}

#[get("orchestra/broadcast")]
pub async fn get_broadcast(state: web::Data<Arc<State>>) -> impl Responder {
    if let Ok(reader) = state.messages.read() {
        let json = serde_json::to_value(reader.clone()).expect("Unable to parse struct to json");
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(json);
    }

    HttpResponse::InternalServerError().into()
}

#[put("orchestra/killswitch")]
pub async fn put_killswitch() -> impl Responder {
    System::current().stop();
    HttpResponse::Ok()
}

#[post("/metric/store/broadcast")]
pub async fn metric_store() -> HttpResponse {
    HttpResponse::Ok().into() 
}

#[post("/metric/process/broadcast")]
pub async fn metric_process() -> HttpResponse {
    HttpResponse::Ok().into() 
}

#[post("/metric/generate/broadcast")]
pub async fn metric_generate() -> HttpResponse {
    HttpResponse::Ok().into() 
}

#[post("/metric/object/broadcast")]
pub async fn metric_object() -> HttpResponse {
    HttpResponse::Ok().into() 
}
