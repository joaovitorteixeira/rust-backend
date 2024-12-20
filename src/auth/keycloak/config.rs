use lazy_static::lazy_static;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, ClientId, ClientSecret, TokenResponse, TokenUrl
};
use std::env;

use crate::api_error::ApiError;

lazy_static! {
    pub static ref KEYCLOAK_REALM: String =
        env::var("KEYCLOAK_REALM").expect("Keycloak realm name is not set");
    pub static ref KEYCLOAK_HOST: String =
        env::var("KEYCLOAK_HOST").expect("Keycloak host not set");
    static ref KEYCLOAK_OAUTH: BasicClient = {
        let keycloak_client_id =
            env::var("KEYCLOAK_CLIENT_ID").expect("Keycloak client id not set");
        let keycloak_client_secret =
            env::var("KEYCLOAK_CLIENT_SECRET").expect("Keycloak client secret not set");

        BasicClient::new(
            ClientId::new(keycloak_client_id),
            Some(ClientSecret::new(keycloak_client_secret)),
            AuthUrl::new(KEYCLOAK_HOST.to_string()).expect("Not possible to set auth URL"),
            Some(
                TokenUrl::new(format!(
                    "{}/realms/{}/protocol/openid-connect/token",
                    KEYCLOAK_HOST.to_string(),
                    KEYCLOAK_REALM.to_string()
                ))
                .expect("Not possible to set token URL"),
            ),
        )
    };
}

pub fn config() -> &'static BasicClient {
    &KEYCLOAK_OAUTH
}

pub async fn get_client_token() -> Result<String, ApiError> {
    let result = config().exchange_client_credentials().request_async(async_http_client).await?;

    Ok(result.access_token().secret().to_string())
}
