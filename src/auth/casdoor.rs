use casdoor_rust_sdk::CasdoorConfig;
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref CASDOOR_CLIENT_ID: String = env::var("CASDOOR_CLIENT_ID").expect("Casdoor client id not set");
    pub static ref CASDOOR_ENDPOINT: String =env::var("CASDOOR_ENDPOINT").expect("Casdoor endpoint not set");
    pub static ref CASDOOR_ORG_NAME: String = env::var("CASDOOR_ORG_NAME").expect("Casdoor org name is not set");
    pub static ref CASDOOR_APP_NAME: String =env::var("CASDOOR_APP_NAME").expect("Casdoor app name is not set"); 
    pub static ref CASDOOR_SESSION_COOKIE_KEY: String = String::from("casdoor_session_id=");
    static ref CASDOOR_CONFIG: CasdoorConfig = {
        let client_secret = env::var("CASDOOR_CLIENT_SECRET").expect("Casdoor client secret is not set");
        let certificate = env::var("CASDOOR_CERTIFICATE").expect("Casdoor certificate is not set");

        CasdoorConfig::new(CASDOOR_ENDPOINT.to_string(), CASDOOR_CLIENT_ID.to_string(), client_secret, certificate, CASDOOR_ORG_NAME.to_string(), Some(CASDOOR_APP_NAME.to_string()))
    };
}

pub fn config() -> &'static CasdoorConfig {
    &CASDOOR_CONFIG
}
