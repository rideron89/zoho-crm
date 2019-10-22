extern crate serde;

use crate::client_error::ClientError;
use crate::token_record::TokenRecord;
use reqwest;
use std::collections::HashMap;
use std::time::Duration;

/// Default network timeout for API requests.
const DEFAULT_TIMEOUT: u64 = 30;

pub struct Client {
    access_token: Option<String>,
    api_domain: Option<String>,
    client_id: String,
    client_secret: String,
    refresh_token: String,
    timeout: u64,
}

impl Client {
    /// Create a new client with an access token, and or client credentials.
    pub fn with_creds(
        access_token: Option<String>,
        api_domain: Option<String>,
        client_id: String,
        client_secret: String,
        refresh_token: String
    ) -> Client {
        Client {
            access_token,
            api_domain,
            client_id,
            client_secret,
            refresh_token,
            timeout: DEFAULT_TIMEOUT,
        }
    }
}

impl Client {
    /// Get the timeout for API requests.
    pub fn timeout(&self) -> u64 {
        self.timeout
    }

    /// Set the timeout for API requests.
    pub fn set_timeout(&mut self, timeout: u64) {
        self.timeout = timeout;
    }

    /// Get the access token.
    pub fn access_token(&self) -> Option<String> {
        self.access_token.clone()
    }

    /// Get the API base URL.
    pub fn api_domain(&self) -> Option<String> {
        self.api_domain.clone()
    }

    /// Get an abbreviated version of the access token.
    pub fn abbreviated_access_token(&self) -> Option<String> {
        match &self.access_token {
            Some(access_token) => {
                let prefix = &access_token[0..9];
                let suffix = &access_token.chars()
                    .rev()
                    .collect::<String>()[0..4]
                    .chars()
                    .rev()
                    .collect::<String>();
                let abbreviated_token = format!("{}..{}", prefix, suffix);

                Some(abbreviated_token)
            },
            None => None
        }
    }
}

impl Client {
    /// Get a new access token from Zoho. Guarantees an access token when it returns an `Result::Ok`.
    pub fn get_new_token(&mut self) -> Result<TokenRecord, ClientError> {
        let url = format!(
            "https://accounts.zoho.com/oauth/v2/token?grant_type=refresh_token&client_id={}&client_secret={}&refresh_token={}",
            self.client_id,
            self.client_secret,
            self.refresh_token
        );

        let client = reqwest::Client::new();
        let mut response = client.post(url.as_str()).send()?;

        let api_response: TokenRecord = response.json()?;

        self.access_token = api_response.access_token.clone();
        self.api_domain = api_response.api_domain.clone();

        match &self.access_token {
            Some(_) => Ok(api_response),
            None => Err(ClientError::from("No token received"))
        }
    }

    /// Make a GET request to the Zoho server.
    pub fn get<T: serde::de::DeserializeOwned>(&mut self, path: &str) -> Result<T, ClientError> {
        if self.access_token.is_none() {
            self.get_new_token()?;
        }

        // we are guaranteed a token when we reach this line
        let token = self.access_token.clone().unwrap();

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(self.timeout))
            .build()?;

        let url = self.api_domain().unwrap() + path;

        let mut response = client
            .get(url.as_str())
            .header("Authorization", String::from("Zoho-oauthtoken ") + &token)
            .send()?;

        let data = response.json()?;

        Ok(data)
    }

    /// Make a POST request to the Zoho server.
    pub fn post(&mut self, path: &str, data: HashMap<String, String>) -> Result<(), ClientError> {
         if self.access_token.is_none() {
            self.get_new_token()?;
        }

        // we are guaranteed a token when we reach this line
        let token = self.access_token.clone().unwrap();

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(self.timeout))
            .build()?;

        let url = self.api_domain().unwrap() + path;

        let mut response = client
            .post(url.as_str())
            .header("Authorization", String::from("Zoho-oauthtoken") + &token)
            .json(&data)
            .send()?;

        let data = response.json()?;

        Ok(data)
    }

    /// Make a PUT request to the Zoho server.
    pub fn put(&mut self, path: &str, data: Vec<HashMap<String, String>>) -> Result<(), ClientError> {
        if self.access_token.is_none() {
            self.get_new_token()?;
        }

        // we are guaranteed a token when we reach this line
        let token = self.access_token.clone().unwrap();

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(self.timeout))
            .build()?;

        let url = self.api_domain().unwrap() + path;

        let mut response = client
            .put(url.as_str())
            .header("Authorization", String::from("Zoho-oauthtoken") + &token)
            .json(&data)
            .send()?;

        let data = response.json()?;

        Ok(data)
    }
}
