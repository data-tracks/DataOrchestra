use actix_web::{get, post, web, HttpResponse, Scope};
use crate::manager::state_manager::{Master, StateManager};

pub fn init_scope() -> Scope {
    web::scope("/init").service(post_init)
}

#[post("")]
async fn post_init(json: web::Json<Master>) -> Result<HttpResponse, actix_web::Error> {
    StateManager::new(json.into_inner());
    Ok(HttpResponse::Ok().finish())
}
