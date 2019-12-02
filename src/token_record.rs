extern crate serde;

use serde::Deserialize;

/// Wrapper around a token sent back from the Zoho service.
///
/// Unless you are saving and/or retrieving a token from somewhere other than Zoho (such as
/// a database), you usually will not need to use this struct.
#[derive(Debug, Deserialize)]
pub struct TokenRecord {
    pub access_token: Option<String>,
    pub api_domain: Option<String>,
    pub error: Option<String>,
    pub expires_in_sec: Option<u64>,
    pub expires_in: Option<u64>,
    pub token_type: Option<String>,
}
