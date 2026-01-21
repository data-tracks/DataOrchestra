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
use data_orchestra_parser::types::generate::ExtGenerate;
use data_orchestra_parser::types::object::ExtObject;
use data_orchestra_parser::types::process::ExtProcess;
use data_orchestra_parser::types::store::ExtStore;
use crate::state::State;

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
    let config = register_items(item.into_inner(), json.into_inner())
        .await
        .map_err(|err| ErrorBadRequest(err))?;

    let mut state_config = state.config.write().await;
    state_config.combine(config);

    let pipeline: Pipeline<(), String> = PipelineBuilder::default()
        .spawner_config(&mut state_config)
        .build()
        .expect("Unable to build pipeline");

    pipeline.run().map_err(|err| ErrorBadRequest(err))?;

    Ok(HttpResponse::Created().finish())
}

pub async fn register_items(json_type: Types, json: Value) -> Result<Config, serde_json::Error> {
    match json_type {
        Types::Config => {
            let new_config: ExtConfig = serde_json::from_value(json)?;
            return Ok(new_config.to_internal());
        }
        Types::Object => {
            let object: Amount<ExtObject> = serde_json::from_value(json)?;

            let object = object.to_internal();
            return Ok(Config {
                object,
                ..Default::default()
            });
        }
        Types::Store => {
            let store: Amount<ExtStore> = serde_json::from_value(json)?;

            let store = store.to_internal();
            return Ok(Config {
                store,
                ..Default::default()
            });
        }
        Types::Process => {
            let process: Amount<ExtProcess> = serde_json::from_value(json)?;

            let process = process.to_internal();
            return Ok(Config {
                process,
                ..Default::default()
            });
        }
        Types::Generate => {
            let generate: Amount<ExtGenerate> = serde_json::from_value(json)?;

            let generate = generate.to_internal();
            return Ok(Config {
                generate,
                ..Default::default()
            });
        }
    }
}
