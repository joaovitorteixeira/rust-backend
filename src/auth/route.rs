use actix_web::{post, web, HttpResponse};
use utoipa_actix_web::service_config::ServiceConfig;

use super::{api, keycloak::user::CreateUser};
use crate::api_error::ApiError;

#[utoipa::path(tag = "auth", 
    responses(
        (status = 201, description = "Create a new user", body= String),
    )
)]
#[post("/sign-up")]
async fn sign_up(create_user: web::Json<CreateUser>) -> Result<HttpResponse, ApiError> {
    let response = api::create(create_user.into_inner()).await?;

    Ok(HttpResponse::Created().body(response))
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(utoipa_actix_web::scope("/auth").service(sign_up));
}
