use std::fmt;

#[derive(Debug)]
pub enum ClientError {
    NeedsToken(String),
    General(String)
}

impl From<String> for ClientError {
    fn from(err: String) -> ClientError {
        ClientError::General(err)
    }
}

impl From<&str> for ClientError {
    fn from(err: &str) -> ClientError {
        ClientError::General(String::from(err))
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

impl From<reqwest::Error> for ClientError {
    fn from(err: reqwest::Error) -> ClientError {
        ClientError::General(err.to_string())
    }
}