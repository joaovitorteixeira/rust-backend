use oauth2::{reqwest::async_http_client, TokenResponse};

use crate::api_error::ApiError;

use super::config::{config};

pub async fn create() -> Result<String, ApiError> {
    let result = config().exchange_client_credentials().request_async(async_http_client).await?;

    Ok(result.access_token().secret().to_string())
}