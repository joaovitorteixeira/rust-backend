use actix_web::{get,  HttpResponse, Responder};
use utoipa_actix_web::service_config::ServiceConfig;
use crate::user::User;

#[utoipa::path(tag = "users", 
    responses(
        (status = 201, description = "Todo created successfully", body = Vec<User>),
    )
)]
#[get("")]
async fn list() -> impl Responder {
    HttpResponse::Ok().json(vec![
        User {id: 1, email: String::from("test@email.com")}
    ])
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(utoipa_actix_web::scope("/users").service(list));
}