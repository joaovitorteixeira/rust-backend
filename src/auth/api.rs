use super::keycloak::user;
use crate::api_error::ApiError;



pub async fn create() -> Result<String, ApiError> {
    let result = user::create().await?;

    Ok(result)
}