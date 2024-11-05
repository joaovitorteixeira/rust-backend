
use actix_web::{get, HttpResponse};
use utoipa_actix_web::service_config::ServiceConfig;
use casdoor_rust_sdk::AuthService;

use crate::api_error::ApiError;

use super::casdoor;

#[utoipa::path(tag = "auth", 
    responses(
        (status = 200),
    )
)]
#[get("/sign-in")]
async fn sign_in() -> Result<HttpResponse, ApiError> {
    let casdoor_config = casdoor::config();
    let auth_service = AuthService::new(casdoor_config);

    let result = auth_service.get_signup_url_enable_password();

    Ok(HttpResponse::Ok().json(result))
}


pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(utoipa_actix_web::scope("/auth").service(sign_in));
}