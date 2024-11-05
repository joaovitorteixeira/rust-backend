use casdoor_rust_sdk::CasdoorConfig;
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    static ref CASDOOR_CONFIG: CasdoorConfig = {
        let endpoint = env::var("CASDOOR_ENDPOINT").expect("Casdoor endpoint not set");
        let client_id = env::var("CASDOOR_CLIENT_ID").expect("Casdoor client id not set");
        let client_secret = env::var("CASDOOR_CLIENT_SECRET").expect("Casdoor client secret is not set");
        let certificate = env::var("CASDOOR_CERTIFICATE").expect("Casdoor certificate is not set");
        let org_name = env::var("CASDOOR_ORG_NAME").expect("Casdoor org name is not set");
        let app_name = env::var("CASDOOR_APP_NAME").expect("Casdoor app name is not set");

        CasdoorConfig::new(endpoint, client_id, client_secret, certificate, org_name, Some(app_name))
    };
}

pub fn config() -> &'static CasdoorConfig {
    &CASDOOR_CONFIG
}
