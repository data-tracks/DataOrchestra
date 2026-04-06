use actix_web::{post, web, HttpResponse, Scope};

pub fn run_scope() -> Scope {
    web::scope("/run").service(post_run)
}

#[post("/")]
async fn post_run() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().finish())
}
