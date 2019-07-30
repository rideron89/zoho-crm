extern crate serde;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct TokenRecord {
    pub access_token: Option<String>,
    pub api_domain: Option<String>,
    pub error: Option<String>,
    pub expires_in_sec: Option<u64>,
    pub expires_in: Option<u64>,
    pub token_type: Option<String>,
}
