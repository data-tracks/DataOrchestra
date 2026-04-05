use data_orchestra_parser::traits::ToInternalVec;
use actix_web::{HttpResponse, Scope, error::ErrorBadRequest, post, web};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;
use data_orchestra_engine::config::Config;
use data_orchestra_engine::pipeline::{Pipeline, PipelineBuilder};
use data_orchestra_parser::amount::Amount;
use data_orchestra_parser::traits::ToInternal;
use data_orchestra_parser::types::config::ExtConfig;
use data_orchestra_parser::types::object::ExtObject;
use crate::state::State;

pub fn register_scope() -> Scope {
    web::scope("/register").service(route_register_item)
}

#[post("/{item}")]
async fn route_register_item(
    json: web::Json<Value>,
    state: web::Data<State>,
) -> Result<HttpResponse, actix_web::Error> {
    let config: ExtConfig = serde_json::from_value(json.into_inner())
        .map_err(|err| ErrorBadRequest(err))?;
    let config = config.to_internal();

    let mut state_config = state.config.write().await;
    state_config.combine(config);

    let pipeline: Pipeline<(), String> = PipelineBuilder::default()
        .spawner_config(&mut state_config)
        .build()
        .expect("Unable to build pipeline");

    pipeline.run().map_err(|err| ErrorBadRequest(err))?;

    Ok(HttpResponse::Created().finish())
}