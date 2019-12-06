use crate::client::ApiErrorResponse;
use std::fmt;

/// Various errors returned by the API.
#[derive(Debug)]
pub enum ClientError {
    /// General error message that encompasses almost any non-token related error message.
    General(String),

    /// Error returned when a response from the API does not deserialize into the user's
    /// custom data type. The raw response will be returned with this error.
    UnexpectedResponseType(String),

    /// Error returned from most API requests.
    ApiError(ApiErrorResponse),
}

impl ClientError {
    /// Return the underlying error message as as string.
    pub fn to_string(&self) -> String {
        match self {
            ClientError::General(error) => error.clone(),
            ClientError::UnexpectedResponseType(error) => error.clone(),
            ClientError::ApiError(error) => error.to_string(),
        }
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClientError::General(error) => write!(f, "{}", error),
            ClientError::UnexpectedResponseType(error) => write!(f, "{}", error),
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