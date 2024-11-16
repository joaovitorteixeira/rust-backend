
use super::keycloak::user::{self, CreateUser};
use crate::api_error::ApiError;




pub async fn create(create_user: CreateUser) -> Result<(), ApiError> {
    user::create(create_user).await?;

    Ok(())
}