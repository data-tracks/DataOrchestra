use actix_web::{get, web, HttpResponse, Scope};
use actix_web::web::get;

pub fn logs_scope() -> Scope {
    web::scope("/logs").service(get_logs)
}

#[get("/")]
async fn get_logs() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().finish())
}