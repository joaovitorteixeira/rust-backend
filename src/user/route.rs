use actix_web::{get, HttpResponse};
use utoipa_actix_web::service_config::ServiceConfig;

use crate::api_error::ApiError;

use super::User;

#[utoipa::path(tag = "users", 
    responses(
        (status = 201, description = "Todo created successfully", body = Vec<User>),
    )
)]
#[get("")]
async fn list() -> Result<HttpResponse, ApiError> {
    let users = User::find_all().await?;

    Ok(HttpResponse::Ok().json(users))
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(utoipa_actix_web::scope("/users").service(list));
}
