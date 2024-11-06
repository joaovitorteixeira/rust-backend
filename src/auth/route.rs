use actix_web::{post, web, HttpResponse};
use utoipa_actix_web::service_config::ServiceConfig;

use crate::api_error::ApiError;

use super::api::{self, CasdoorUser, SignInPasswordMethod};

#[utoipa::path(tag = "auth", 
    responses(
        (status = 200, description = "Login the user", body= CasdoorUser),
    )
)]
#[post("/sign-in")]
async fn sign_in(sign_in_data: web::Json<SignInPasswordMethod>) -> Result<HttpResponse, ApiError> {
    let response = api::sign_in(sign_in_data.into_inner()).await?;
    let session_id = api::get_session_id_from_response(&response).await.expect("Invalid session cookie from response");
    let user: api::CasdoorUser = api::get_user(session_id).await?;
    Ok(HttpResponse::Ok().json(user))
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(utoipa_actix_web::scope("/auth").service(sign_in));
}
