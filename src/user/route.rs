use actix_web::{get, post, web, HttpResponse};
use utoipa_actix_web::service_config::ServiceConfig;

use crate::api_error::ApiError;

use super::{model::UserCreate, User};

#[utoipa::path(tag = "users", 
    responses(
        (status = 200, description = "List all the users in the system", body = Vec<User>),
    )
)]
#[get("")]
async fn list() -> Result<HttpResponse, ApiError> {
    let users = User::list().await?;

    Ok(HttpResponse::Ok().json(users))
}

#[utoipa::path(tag = "users", responses(
        (status = 204, description = "Creates a new user, but does not return them")
    )
)]
#[post("")]
async fn create(user: web::Json<UserCreate>) -> Result<HttpResponse, ApiError> {
    User::create(user.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        utoipa_actix_web::scope("/users")
            .service(list)
            .service(create),
    );
}
