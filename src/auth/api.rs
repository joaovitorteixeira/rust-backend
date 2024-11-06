use core::fmt;

use reqwest::{header::{COOKIE, SET_COOKIE}, Response};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::casdoor::{
    CASDOOR_APP_NAME, CASDOOR_CLIENT_ID, CASDOOR_ENDPOINT, CASDOOR_ORG_NAME,
    CASDOOR_SESSION_COOKIE_KEY,
};
use crate::api_error::ApiError;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SignInPasswordMethod {
    username: String,
    password: String,
}

#[derive(Serialize)]
enum SignInMethod {
    Password,
}

impl fmt::Display for SignInMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Password => "Password",
            }
        )
    }
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum SignInType {
    Login,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CasdoorUser {
    id: String,
    name: String,
    email: String,
    email_verified_at: Option<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Serialize)]
pub struct CasdoorCommonFields {
    application: String,
    organization: String,
    auto_signin: bool,
    signin_method: SignInMethod,
    r#type: SignInType,
}

#[derive(Serialize)]
pub struct CasdoorSignIn<T> {
    #[serde(flatten)]
    common_fields: CasdoorCommonFields,
    #[serde(flatten)]
    data: T,
}

pub async fn sign_in(sing_in_data: SignInPasswordMethod) -> Result<Response, ApiError> {
    let client = reqwest::Client::new();
    let casdoor_sign_in = CasdoorSignIn {
        common_fields: CasdoorCommonFields {
            application: CASDOOR_APP_NAME.to_string(),
            organization: CASDOOR_ORG_NAME.to_string(),
            auto_signin: true,
            signin_method: SignInMethod::Password,
            r#type: SignInType::Login,
        },
        data: sing_in_data,
    };
    let url = format!(
        "{}/login?clientId={}?responseType=code?redirectUri=localhost%3A8080",
        CASDOOR_ENDPOINT.to_string(),
        CASDOOR_CLIENT_ID.to_string()
    );
    let result = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&casdoor_sign_in)
        .send()
        .await?;

    Ok(result)
}

pub async fn get_user(session_id: String) -> Result<CasdoorUser, ApiError> {
    let client = reqwest::Client::new();
    let url = format!("{}/user", CASDOOR_ENDPOINT.to_string());

    let result = client
        .get(url)
        .header(
            COOKIE,
            format!("{}{}", CASDOOR_SESSION_COOKIE_KEY.to_string(), session_id),
        )
        .header("Content-Type", "application/json")
        .send()
        .await?;

    Ok(result.json::<CasdoorUser>().await?)
}

pub async fn get_session_id_from_response(response: &reqwest::Response) -> Option<String> {
    if let Some(set_cookie_header) = response.headers().get(SET_COOKIE) {
        let set_cookie_str = set_cookie_header
            .to_str()
            .expect("Invalid response with no cookie");
        for cookie in set_cookie_str.split(';') {
            if let Some(session_id) = cookie
                .trim()
                .strip_prefix(&CASDOOR_SESSION_COOKIE_KEY.to_string())
            {
                return Some(session_id.to_string());
            }
        }
    }

    None
}
