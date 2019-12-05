use crate::client::ApiErrorResponse;
use std::fmt;

/// Various errors returned by the API.
#[derive(Debug)]
pub enum ClientError {
    /// Error identifying that the previous request was not completed because the access token
    /// is either invalid or expired.
    NeedsToken(String),

    /// General error message that encompasses almost any non-token related error message.
    General(String),

    /// Error returned from most API requests.
    ApiError(ApiErrorResponse),
}

impl ClientError {
    /// Return the underlying error message as as string.
    pub fn to_string(&self) -> String {
        match self {
            ClientError::NeedsToken(error) => error.clone(),
            ClientError::General(error) => error.clone(),
            ClientError::ApiError(error) => error.to_string(),
        }
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClientError::NeedsToken(error) => write!(f, "{}", error),
            ClientError::General(error) => write!(f, "{}", error),
            ClientError::ApiError(error) => write!(f, "{}", error.to_string()),
        }
    }
}

impl From<String> for ClientError {
    fn from(err: String) -> ClientError {
        ClientError::General(err)
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(err: serde_json::Error) -> Self {
        ClientError::General(err.to_string())
    }
}

impl From<serde_urlencoded::ser::Error> for ClientError {
    fn from(err: serde_urlencoded::ser::Error) -> Self {
        ClientError::General(err.to_string())
    }
}

impl From<&str> for ClientError {
    fn from(err: &str) -> ClientError {
        ClientError::General(String::from(err))
    }
}

impl From<reqwest::Error> for ClientError {
    fn from(err: reqwest::Error) -> ClientError {
        ClientError::General(err.to_string())
    }
}