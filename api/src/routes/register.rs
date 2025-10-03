use std::{fmt::Display, sync::Arc};

use actix_web::{HttpResponse, Responder, Scope, error::ErrorBadRequest, post, web};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;

use crate::state::State;

use data_orchestra_core::{
    interface::{
        config::ExtConfig, generate::ExtGenerate, object::ExtObject, process::ExtProcess,
        store::ExtStore,
    },
    shared::{Amount, ToInternal, ToInternalVec},
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Types {
    Config,
    Object,
    Generate,
    Process,
    Store,
}

impl Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Config => "config",
            Self::Generate => "generate",
            Self::Object => "object",
            Self::Process => "process",
            Self::Store => "store",
        };

        write!(f, "{}", s)
    }
}

pub fn register_scope() -> Scope {
    web::scope("/register").service(route_register_item)
}

#[post("/{item}")]
async fn route_register_item(
    item: web::Path<Types>,
    json: web::Json<Value>,
    state: web::Data<State>,
) -> Result<HttpResponse, actix_web::Error> {
    register_item(item.into_inner(), json.into_inner(), state.into_inner())
        .await
        .map_err(|err| ErrorBadRequest(err))?;
    Ok(HttpResponse::Created().finish())
}

pub async fn register_item(
    json_type: Types,
    json: Value,
    state: Arc<State>,
) -> Result<(), serde_json::Error> {
    info!("Adding new item of type {json_type}");

    match json_type {
        Types::Config => {
            let new_config: ExtConfig = serde_json::from_value(json)?;

            let new_config = new_config.to_internal();
            let mut config = state.config.write().await;
            config.combine(new_config);
        }
        Types::Object => {
            let object: Amount<ExtObject> = serde_json::from_value(json)?;

            let object = object.to_internal();
            let mut config = state.config.write().await;
            config.object.extend(object);
        }
        Types::Store => {
            let store: Amount<ExtStore> = serde_json::from_value(json)?;

            let store = store.to_internal();
            let mut config = state.config.write().await;
            config.store.extend(store);
        }
        Types::Process => {
            let process: Amount<ExtProcess> = serde_json::from_value(json)?;

            let process = process.to_internal();
            let mut config = state.config.write().await;
            config.process.extend(process);
        }
        Types::Generate => {
            let generate: Amount<ExtGenerate> = serde_json::from_value(json)?;

            let generate = generate.to_internal();
            let mut config = state.config.write().await;
            config.generate.extend(generate);
        }
    }

    info!("Successfully added item of type {json_type}");

    Ok(())
}

#[post("/config")]
async fn register(config: web::Json<ExtConfig>, state: web::Data<Arc<State>>) -> impl Responder {
    return HttpResponse::Ok();
}
