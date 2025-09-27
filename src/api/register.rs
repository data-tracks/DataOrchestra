use std::{fmt::Display, str::FromStr, sync::Arc};

use actix_web::{
    HttpResponse, HttpResponseBuilder, Responder, Scope, error::ErrorBadRequest, post, web,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    api::state::State,
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

pub fn register_scope() -> Scope {
    web::scope("/register").service(register_item)
}

#[post("/{item}")]
async fn register_item(
    item: web::Path<Types>,
    json: web::Json<Value>,
    state: web::Data<Arc<State>>,
) -> Result<HttpResponse, actix_web::Error> {
    match item.into_inner() {
        Types::Config => {
            let new_config: ExtConfig =
                serde_json::from_value(json.into_inner()).map_err(|e| ErrorBadRequest(e))?;

            let (new_config, _portainer) = new_config.to_internal();
            let mut config = state.config.write().await;
            config.combine(new_config);
        }
        Types::Object => {
            let object: Amount<ExtObject> =
                serde_json::from_value(json.into_inner()).map_err(|e| ErrorBadRequest(e))?;

            let object = object.to_internal();
            let mut config = state.config.write().await;
            config.object.extend(object);
        }
        Types::Store => {
            let store: Amount<ExtStore> =
                serde_json::from_value(json.into_inner()).map_err(|e| ErrorBadRequest(e))?;

            let store = store.to_internal();
            let mut config = state.config.write().await;
            config.store.extend(store);
        }
        Types::Process => {
            let process: Amount<ExtProcess> =
                serde_json::from_value(json.into_inner()).map_err(|e| ErrorBadRequest(e))?;

            let process = process.to_internal();
            let mut config = state.config.write().await;
            config.process.extend(process);
        }
        Types::Generate => {
            let generate: Amount<ExtGenerate> =
                serde_json::from_value(json.into_inner()).map_err(|e| ErrorBadRequest(e))?;

            let generate = generate.to_internal();
            let mut config = state.config.write().await;
            config.generate.extend(generate);
        }
    }

    Ok(HttpResponse::Created().finish())
}

#[post("/config")]
async fn register(config: web::Json<ExtConfig>, state: web::Data<Arc<State>>) -> impl Responder {
    return HttpResponse::Ok();
}
