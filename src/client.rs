extern crate serde;
extern crate serde_json;

use crate::client_error::ClientError;
use crate::token_record::TokenRecord;
use reqwest;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Duration;

#[cfg(test)]
use mockito;

/// Default network timeout for API requests.
const DEFAULT_TIMEOUT: u64 = 30;

/// Handles making requests to the v2 of the Zoho API.
///
/// You can either create this client with a pre-determined access token, or fetch
/// a new one later on. You will need an API client ID, secret, and refresh token.
///
/// You can read more information here:
/// https://www.zoho.com/crm/developer/docs/api/oauth-overview.html
///
/// You can create a client with the `with_creds()` method. The method also allows
/// you to supply an access token and API domain if you are pulling them from
/// somewhere else, such as a database.
///
/// ### Example
///
/// ```
/// use zoho::ZohoClient;
///
/// let client_id = "YOUR_CLIENT_ID";
/// let client_secret = "YOUR_CLIENT_SECRET";
/// let refresh_token = "YOUR_REFRESH_TOKEN";
///
/// let client = ZohoClient::with_creds(
///     None, // access token
///     None, // api domain
///     String::from(client_id),
///     String::from(client_secret),
///     String::from(refresh_token)
/// );
/// ```
pub struct Client {
    access_token: Option<String>,
    api_domain: Option<String>,
    client_id: String,
    client_secret: String,
    refresh_token: String,
    timeout: u64,
}

impl Client {
    /// Create a new client.
    ///
    /// You can supply an optional access token and/or api domain. However, you must supply
    /// a client ID, secret, and refresh token.
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
    /// Get the timeout (in seconds) for API requests. Default is 30 seconds.
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

    /// Get the API domain URL.
    pub fn api_domain(&self) -> Option<String> {
        self.api_domain.clone()
    }

    /// Get an abbreviated version of the access token. This is a (slightly) safer version
    /// of the access token should you need to print it out.
    ///
    /// ```
    /// # use zoho::ZohoClient;
    /// let token = "1000.ad8f97a9sd7f9a7sdf7a89s7df87a9s8.a77fd8a97fa89sd7f89a7sdf97a89df3";
    /// # let client_id = String::from("YOUR_CLIENT_ID");
    /// # let client_secret = String::from("YOUR_CLIENT_SECRET");
    /// # let refresh_token = String::from("YOUR_REFRESH_TOKEN");
    ///
    /// # let mut client = ZohoClient::with_creds(Some(token.to_string()), None, client_id, client_secret, refresh_token);
    ///
    /// assert_eq!("1000.ad8f..9df3".to_string(), client.abbreviated_access_token().unwrap());
    /// ```
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
    /// Get the API base path, which changes depending on the current environment.
    ///
    /// This is primarily used to allow for HTTP test mocking of API calls.
    fn get_api_base_path() -> String {
        #[cfg(test)]
        return mockito::server_url();

        #[cfg(not(test))]
        return String::from("https://accounts.zoho.com");
    }

    /// Get a new access token from Zoho. Guarantees an access token when it returns
    /// an `Result::Ok`.
    ///
    /// The access token is saved to the `ZohoClient`, so you don't
    /// need to retrieve the token and set it in different steps. But a copy
    /// of it is returned by this method.
    pub fn get_new_token(&mut self) -> Result<TokenRecord, ClientError> {
        let url = format!(
            "{}/oauth/v2/token?grant_type=refresh_token&client_id={}&client_secret={}&refresh_token={}",
            Client::get_api_base_path(),
            self.client_id,
            self.client_secret,
            self.refresh_token
        );

        let client = reqwest::Client::new();
        let mut response = client.post(url.as_str()).send()?;
        let raw_response = response.text()?;

        // TODO: refactor this with a more idiomatic pattern
        if let Ok(response) = serde_json::from_str::<AuthErrorResponse>(&raw_response) {
            return Err(ClientError::General(response.error));
        }

        let api_response: TokenRecord = serde_json::from_str(&raw_response)?;

        self.access_token = api_response.access_token.clone();
        self.api_domain = api_response.api_domain.clone();

        match &self.access_token {
            Some(_) => Ok(api_response),
            None => Err(ClientError::from("No token received"))
        }
    }

    /// Make a GET request to the Zoho server.
    ///
    /// TODO: needs to handle error responses from Zoho.
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
        let raw_response = response.text()?;

        if let Ok(response) = serde_json::from_str::<ApiErrorResponse>(&raw_response) {
            return Err(ClientError::General(response.code));
        }

        match serde_json::from_str::<T>(&raw_response) {
            Ok(data) => Ok(data),
            Err(_) => {
                if raw_response.len() > 0 {
                    Err(ClientError::General(raw_response))
                } else {
                    Err(ClientError::General(String::from("Unknown error response returned")))
                }
            },
        }
    }

    /// Make a POST request to the Zoho server.
    ///
    /// TODO: needs to handle error responses from Zoho.
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
    ///
    /// TODO: needs to handle error responses from Zoho.
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

/// This is one possible error response that Zoho might send back when requesting a token. If
/// the API response contains an `error` field, it will be treated as an `AuthErrorResponse`
/// and should be handled accordingly.
#[derive(Deserialize)]
struct AuthErrorResponse {
    error: String,
}

/// This is one possible error response that Zoho might send back from an API request. It is
/// different than the response format given back when requesting a token. `code` will be an
/// identifier for the type of error, while the `_message` field *might* have more information.
///
/// `status` will return a text status: "error" on error.
///
/// There is also a `data` field we are not capturing.
#[derive(Deserialize)]
struct ApiErrorResponse {
    code: String,

    #[serde(alias = "message")]
    _message: String,

    #[serde(alias = "status")]
    _status: String,
}

#[cfg(test)]
mod tests {
    extern crate mockito;

    use mockito::{mock, Matcher, Mock};
    use super::Client;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct Response {
        data: Vec<ResponseDataItem>,
        info: ResponseInfo,
    }

    #[derive(Debug, Deserialize)]
    struct ResponseDataItem {
        id: String,
    }

    #[derive(Debug, Deserialize)]
    struct ResponseInfo {
        more_records: bool,
        per_page: usize,
        count: usize,
        page: usize,
    }

    /// Get a `Client` with an access token.
    fn get_client(access_token: Option<String>, api_domain: Option<String>) -> Client {
        let id = String::from("id");
        let secret = String::from("secret");
        let refresh_token = String::from("refresh_token");

        Client::with_creds(access_token, api_domain, id, secret, refresh_token)
    }

    /// Get an HTTP mocker.
    fn get_mocker<T: Into<Matcher>>(method: &str, url: T, body: Option<&str>) -> Mock {
        let mut mocker = mock(method, url)
            .with_status(200)
            .with_header("Content-Type", "application/json;charset=UTF-8");

        if let Some(body) = body {
            mocker = mocker
                .with_header("Content-Length", &body.to_string().len().to_string())
                .with_body(body);
        }

        mocker = mocker.create();

        mocker
    }

    #[test]
    fn no_access_token() {
        let client = get_client(None, Some(String::from("api_domain")));

        assert_eq!(client.access_token(), None);
    }

    #[test]
    fn no_domain() {
        let client = get_client(Some(String::from("access_token")), None);

        assert_eq!(client.api_domain(), None);
    }

    #[test]
    fn preset_access_token() {
        let access_token = String::from("access_token");
        let client = get_client(Some(access_token.clone()), None);

        assert_eq!(client.access_token(), Some(access_token));
    }

    #[test]
    fn preset_api_domain() {
        let domain = String::from("api_domain");
        let client = get_client(None, Some(domain.clone()));

        assert_eq!(client.api_domain(), Some(domain));
    }

    #[test]
    fn empty_abbreviated_token() {
        let client = get_client(None, None);

        assert_eq!(client.abbreviated_access_token(), None);
    }

    #[test]
    fn valid_abbreviated_token() {
        let access_token = String::from("12345678901234567890");
        let client = get_client(Some(access_token), None);

        assert_ne!(client.access_token().unwrap().len(), 15);
        assert_eq!(client.abbreviated_access_token().unwrap().len(), 15);
    }

    #[test]
    /// Tests that a valid token is set after calling the `Client` `get_new_token()` method.
    fn get_new_token_success() {
        let access_token = "9999.bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let api_domain = "https://www.zohoapis.com";
        let body = format!("{{\"access_token\":\"{}\",\"expires_in_sec\":3600,\"api_domain\":\"{}\",\"token_type\":\"Bearer\",\"expires_in\":3600000}}", access_token, api_domain);
        let mocker = get_mocker("POST", Matcher::Any, Some(&body));
        let mut client = get_client(None, None);

        match client.get_new_token() {
            Ok(e) => println!("Good: {:#?}", e),
            Err(error) => println!("Bad: {:#?}", error),
        }

        mocker.assert();
        assert_eq!(client.access_token(), Some(access_token.to_string()));
    }

    #[test]
    /// Tests that a valid API domain is set after calling the `Client` `get_new_token()` method.
    fn get_new_api_domain_success() {
        let access_token = "9999.bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let api_domain = "https://www.zohoapis.com";
        let body = format!(r#"{{"access_token":"{}","expires_in_sec":3600,"api_domain":"{}","token_type":"Bearer","expires_in":3600000}}"#, access_token, api_domain);
        let mocker = get_mocker("POST", Matcher::Any, Some(&body));
        let mut client = get_client(None, None);

        client.get_new_token().unwrap();

        mocker.assert();
        assert_eq!(client.api_domain(), Some(api_domain.to_string()));
    }

    #[test]
    /// Tests that an error is return after calling the `Client` `get_new_token()` method with an
    /// invalid refresh token.
    fn get_new_token_invalid_token() {
        let error_message = "invalid_token";
        let body = format!(r#"{{"error":"{}"}}"#, error_message);
        let mocker = get_mocker("POST", Matcher::Any, Some(&body));
        let mut client = get_client(None, None);

        match client.get_new_token() {
            Ok(_) => panic!("Error was not thrown"),
            Err(error) => {
                assert_eq!(error_message.to_string(), error.to_string());
            }
        }

        mocker.assert();
    }

    #[test]
    /// Tests that a `TokenRecord` with a valid access token is returned from the `Client`
    /// `get_new_token()` method.
    fn return_new_token_success() {
        let access_token = "9999.bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let api_domain = "https://www.zohoapis.com";
        let body = format!(r#"{{"access_token":"{}","expires_in_sec":3600,"api_domain":"{}","token_type":"Bearer","expires_in":3600000}}"#, access_token, api_domain);
        let mocker = get_mocker("POST", Matcher::Any, Some(&body));
        let mut client = get_client(None, None);

        let token = client.get_new_token().unwrap();

        mocker.assert();
        assert_eq!(token.access_token, Some(access_token.to_string()));
    }

    #[test]
    /// Tests that a `TokenRecord` with a valid API domain is returned from the `Client`
    /// `get_new_token()` method.
    fn return_api_domain_success() {
        let access_token = "9999.bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let api_domain = "https://www.zohoapis.com";
        let body = format!(r#"{{"access_token":"{}","expires_in_sec":3600,"api_domain":"{}","token_type":"Bearer","expires_in":3600000}}"#, access_token, api_domain);
        let mocker = get_mocker("POST", Matcher::Any, Some(&body));
        let mut client = get_client(None, None);

        let token = client.get_new_token().unwrap();

        mocker.assert();
        assert_eq!(token.api_domain, Some(api_domain.to_string()));
    }

    #[test]
    fn get_success() {
        let access_token = "9999.bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let api_domain = mockito::server_url();
        let record_id = "40000000123456789";
        let body = format!(r#"{{"data":[{{"id":"{}"}}],"info":{{"more_records":true,"per_page":1,"count":1,"page":1}}}}"#, record_id);
        let mocker = get_mocker("GET", Matcher::Any, Some(&body));
        let mut client = get_client(Some(String::from(access_token)), Some(String::from(api_domain)));

        let response: Response = client.get("/crm/v2/Accounts?page=1&per_page=1").unwrap();

        mocker.assert();
        assert_eq!(response.data.get(0).unwrap().id, record_id);
    }

    #[test]
    fn get_regular_error() {
        let access_token = "9999.bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let api_domain = mockito::server_url();
        let error_code = "INVALID_URL_PATTERN";
        let body = format!(r#"{{"code":"{}","details":{{}},"message":"Please check if the URL trying to access is a correct one","status":"error"}}"#, error_code);
        let mocker = get_mocker("GET", Matcher::Any, Some(&body));
        let mut client = get_client(Some(String::from(access_token)), Some(String::from(api_domain)));

        match client.get::<Response>("/crm/v2/INVALIDMODULE?page=1&per_page=1") {
            Ok(_) => panic!("Response did not return an error"),
            Err(err) => {
                assert_eq!(err.to_string(), error_code.to_string());
            }
        }

        mocker.assert();
    }

    #[test]
    fn get_text_error() {
        let access_token = "9999.bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let api_domain = mockito::server_url();
        let error_code = "invalid_client";
        let body = format!("{}", error_code);
        let mocker = get_mocker("GET", Matcher::Any, Some(&body));
        let mut client = get_client(Some(String::from(access_token)), Some(String::from(api_domain)));

        match client.get::<Response>("/crm/v2/INVALIDMODULE?page=1&per_page=1") {
            Ok(_) => panic!("Response did not return an error"),
            Err(err) => {
                assert_eq!(err.to_string(), error_code.to_string());
            }
        }

        mocker.assert();
    }
}
