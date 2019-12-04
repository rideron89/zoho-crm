use std::fmt;

#[derive(Debug)]
pub enum ClientError {
    NeedsToken(String),
    General(String)
}

impl ClientError {
    pub fn to_string(&self) -> String {
        match self {
            ClientError::NeedsToken(error) => error.clone(),
            ClientError::General(error) => error.clone(),
        }
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClientError::NeedsToken(desc) => write!(f, "{}", desc),
            ClientError::General(desc) => write!(f, "{}", desc)
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