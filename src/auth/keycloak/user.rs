use reqwest::{Client, Response};
use serde::Deserialize;
use serde_json::json;
use utoipa::ToSchema;

use crate::api_error::ApiError;

use super::config::{get_client_token, KEYCLOAK_HOST, KEYCLOAK_REALM};

#[derive(Deserialize, ToSchema)]
pub struct CreateUser {
    email: String,
    first_name: String,
    last_name: String,
    password: String,
}

pub async fn create(create_user: CreateUser) -> Result<(), ApiError> {
    let client_token = get_client_token().await?;
    let create_user_url = format!(
        "{}/admin/realms/{}/users",
        KEYCLOAK_HOST.to_string(),
        KEYCLOAK_REALM.to_string()
    );
    let user_data = json!({
        "email": create_user.email,
        "username": create_user.email,
        "firstName": create_user.first_name,
        "lastName": create_user.last_name,
        "enabled": true,
        "credentials": [{"value": create_user.password, "type": "password"}]
    });
    let client = Client::new();

    let response = client
        .post(create_user_url)
        .bearer_auth(client_token)
        .json(&user_data)
        .send()
        .await?;

    let _ = remove_error_from_response(response).await?;

    Ok(())
}

pub async fn remove_error_from_response(response: Response) -> Result<Response, ApiError> {
    if response.status().is_success() {
        Ok(response)
    } else {
        Err(ApiError::new(
            response.status().as_u16(),
            response.text().await?,
        ))
    }
}
